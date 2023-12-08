use {
	crate::{
		api::{
			self,
			piston::versions::Response as Versions,
		},
		spec::manifest::{
			Arch,
			ArgsContainer,
			AssetIndex,
			Library,
			Manifest,
			ModernArgs,
			NativeManifest,
			Os,
			Rule,
			RuleAction,
			RuleCondition,
		},
		Error,
	},
	download::Item,
	serde::de::DeserializeOwned,
	std::{
		future::Future,
		io::ErrorKind,
		path::Path,
	},
	tokio::{
		fs::{
			self,
			File,
		},
		io::{
			AsyncReadExt,
			AsyncWriteExt,
		},
	},
};

async fn read_or_fetch<T, F, R>(
	path: &Path,
	items: &mut Vec<Item>,
	item: Item,
	fetch: F,
) -> Result<T, Error>
where
	T: DeserializeOwned,
	F: FnOnce() -> R,
	R: Future<Output = Result<T, reqwest::Error>>,
{
	Ok(match File::open(path).await {
		Ok(mut it) => {
			let mut content = String::new();
			it.read_to_string(&mut content).await?;
			serde_json::from_str(&content)?
		}
		Err(err) => match err.kind() {
			ErrorKind::NotFound => {
				let result = fetch().await?;
				items.push(item);
				result
			}
			_ => return Err(err.into()),
		},
	})
}

fn unpack_rule(rule: &Rule) -> bool {
	let mut allow = true;

	match &rule.condition {
		Some(RuleCondition::Os { name, arch, .. }) => {
			if let Some(os_name) = &name {
				allow = os_name == &Os::target().unwrap()
			}

			if let Some(os_arch) = &arch {
				allow = os_arch == &Arch::target()
			}
		}
		_ => {}
	};

	match rule.action {
		RuleAction::Allow => allow,
		RuleAction::Disallow => !allow,
	}
}

pub async fn install(root: &Path, id: &str) -> Result<(), Error> {
	let mut items: Vec<Item> = Vec::new();

	let version_dir = root.join("versions").join(id);
	let assets_dir = root.join("assets");
	let asset_indexes_dir = assets_dir.join("indexes");
	let asset_objects_dir = assets_dir.join("objects");
	let libraries_dir = root.join("libraries");

	let versions_list_lookup_path = root.join("version_manifest_v2.json");
	let manifest_lookup_path = version_dir.join(format!("{id}.json"));

	let manifest: Manifest = serde_json::from_str(&fs::read_to_string(&manifest_lookup_path).await?)?;

	let mut modified = false;

	let manifest = match manifest {
		Manifest::Native(it) => it,
		Manifest::Inherited(it) => {
			let versions_cache_item = Item {
				path: versions_list_lookup_path.clone(),
				url: api::piston::versions::URL,
				known_size: None,
				known_sha: None,
				ignore_integrity: true,
			};

			let versions_list: Versions = read_or_fetch(
				&versions_list_lookup_path,
				&mut items,
				versions_cache_item,
				|| api::piston::versions::get(),
			)
			.await?;

			let version_meta =
				versions_list
					.into_iter()
					.find(|it| it.id == id)
					.ok_or(Error::Inconsistent(
						"Requested version not found in list".into(),
					))?;

			let manifest_cache_item = Item {
				path: manifest_lookup_path.clone(),
				url: version_meta.url.clone(),
				known_sha: Some(version_meta.sha1),
				known_size: None,
				ignore_integrity: true,
			};

			let mut manifest: Box<NativeManifest> = read_or_fetch(
				&manifest_lookup_path,
				&mut items,
				manifest_cache_item,
				|| async {
					reqwest::get(version_meta.url)
						.await?
						.json()
						.await
						.map(Box::new)
				},
			)
			.await?;

			manifest.id = it.id;
			manifest.main_class = it.main_class;
			manifest.version_type = it.version_type;

			manifest.arguments = manifest.arguments.into();
			let inherited_args: ModernArgs = it.arguments.into();

			match &mut manifest.arguments {
				ArgsContainer::Modern(it) => {
					it.arguments.jvm.extend(inherited_args.arguments.jvm);
					it.arguments.game.extend(inherited_args.arguments.game);
				}
				_ => {}
			}

			manifest.libraries.extend(it.libraries);

			modified = true;

			manifest
		}
	};

	if modified {
		let mut file = File::create(&manifest_lookup_path).await?;
		file.write_all(&serde_json::to_vec(&manifest)?).await?;
	}

	let asset_index: Box<AssetIndex> = {
		let asset_index = manifest.asset_index;

		let asset_index_item = Item {
			path: manifest_lookup_path.clone(),
			url: asset_index.url.clone(),
			known_sha: Some(asset_index.sha1.clone()),
			known_size: Some(asset_index.size),
			ignore_integrity: false,
		};

		read_or_fetch(
			&asset_indexes_dir.join(format!("{}.json", manifest.assets)),
			&mut items,
			asset_index_item,
			|| async {
				reqwest::get(asset_index.url)
					.await?
					.json()
					.await
					.map(Box::new)
			},
		)
		.await?
	};

	for lib in manifest.libraries {
		use Library::*;

		match lib {
			Common(it) => {
				let artifact = it.downloads.artifact;

				items.push(Item {
					url: artifact.url,
					path: libraries_dir.join(artifact.path),
					known_size: Some(artifact.size),
					known_sha: Some(artifact.sha1),
					ignore_integrity: false,
				})
			}
			Seminative(it) => {
				if it.rules.iter().all(unpack_rule) {
					let artifact = it.downloads.artifact;

					items.push(Item {
						url: artifact.url,
						path: libraries_dir.join(artifact.path),
						known_size: Some(artifact.size),
						known_sha: Some(artifact.sha1),
						ignore_integrity: false,
					})
				}
			}
			Native(mut it) => {
				if it.rules.iter().all(unpack_rule) {
					let common_artifact = it.downloads.artifact;

					items.push(Item {
						url: common_artifact.url,
						path: libraries_dir.join(common_artifact.path),
						known_size: Some(common_artifact.size),
						known_sha: Some(common_artifact.sha1),
						ignore_integrity: false,
					});

					let classifier = it
						.natives
						.get(&Os::target().unwrap())
						.ok_or(Error::InvalidManifest("Missing native classifier".into()))?;

					let classifier_artifact = it.downloads.classifiers.remove(classifier).ok_or(Error::InvalidManifest("Missing native classifier".into()))?;

					items.push(Item {
						url: classifier_artifact.url,
						path: libraries_dir.join(classifier_artifact.path),
						known_size: Some(classifier_artifact.size),
						known_sha: Some(classifier_artifact.sha1),
						ignore_integrity: false,
					});
				}
			}
			Custom(it) => {
				let path = it.name.to_path();

				items.push(Item {
					url: it.url.join(path.to_str().ok_or(Error::InvalidUtf8Path)?)?,
					path: libraries_dir.join(path),
					known_size: None,
					known_sha: None,
					ignore_integrity: true,
				});
			}
		}
	}

	for it in asset_index.objects.into_values() {
		let path = format!("{}/{}", &it.hash[..2], it.hash);

		items.push(Item {
			url: api::resources::make_url(&it.hash),
			path: asset_objects_dir.join(path),
			known_size: Some(it.size),
			known_sha: Some(it.hash),
			ignore_integrity: false,
		})
	}

	{
		let path = root.join("jre_components.json");

		let item = Item {
			url: api::launchermeta::runtime::URL,
			path,
			known_size: None,
			known_sha: None,
			ignore_integrity: true,
		};
	}

	// let current_target

	todo!()
}
