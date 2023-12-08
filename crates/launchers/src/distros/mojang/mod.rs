use reqwest_middleware::ClientBuilder;
use {
	crate::{
		distros::{
			mojang::api::get_jre_components,
			Distro,
			VersionId,
		},
		launch::{
			utils,
			ProcessLauncher,
			TryIntoLauncher,
		},
		specs::prelude::*,
		Error,
	},
	http_cache_reqwest::{
		CACacheManager,
		Cache,
		CacheMode,
		HttpCache,
		HttpCacheOptions,
	},
	once_cell::sync::Lazy,
	reqwest::{
		Client, ,
	},
	serde::Deserialize,
	std::{
		collections::HashSet,
		path::PathBuf,
	},
	url::Url,
};

#[cfg(feature = "install")]
use crate::install::action::{
	Action,
	DownloadAction,
	WriteAction,
};

mod api;

#[derive(Debug, Clone)]
pub struct Mojang(Manifest);

static DEFAULT_FEATURES: Lazy<HashSet<&str>> =
	Lazy::new(|| HashSet::from(["has_custom_resolution"]));

#[derive(Debug, Clone, Deserialize)]
pub struct VersionOptions {
	mcv: String,
}

impl VersionId for VersionOptions {
	fn version_id(&self) -> String {
		self.mcv.clone()
	}
}

impl Distro for Mojang {
	type Error = Error;
	type VersionOptions = VersionOptions;

	fn from_manifest(value: Manifest) -> Self {
		Self(value)
	}

	fn try_from_json(value: &str) -> Result<Self, Self::Error> {
		Ok(Self(serde_json::from_str(value)?))
	}

	async fn fetch_manifest(options: &Self::VersionOptions) -> Result<Manifest, Self::Error> {
		let versions = api::get_versions().await?.versions;
		let version = versions
			.into_iter()
			.find(|it| it.id == options.mcv)
			.ok_or(Error::ManifestNotFound)?;

		Ok(reqwest::get(version.url).await?.json().await?)
	}
}

impl TryIntoLauncher for Mojang {
	type Error = Error;

	fn try_into_launcher(self) -> Result<ProcessLauncher, Error> {
		let manifest = match self.0 {
			Manifest::Root(it) => it,
			Manifest::Inherited(_) => return Err(Error::Inherited),
		};

		let classpath = manifest.libraries.into_classpath()?;

		let ModernArgs { arguments }: ModernArgs = manifest.arguments.into();

		let jre_binaries = PathBuf::from("jre")
			.join(manifest.java_version.component)
			.join("bin");

		let mut process = ProcessLauncher {
			bin: jre_binaries.join(utils::BINARY_NAME),
			main_class: manifest.main_class,
			..Default::default()
		};

		utils::process_args(arguments.jvm, &mut process.jvm_args, &DEFAULT_FEATURES);
		utils::process_args(arguments.game, &mut process.game_args, &DEFAULT_FEATURES);

		process
			.jvm_args
			.push("-Dminecraft.api.auth.host=${minecraft_auth_host}".into());
		process
			.jvm_args
			.push("-Dminecraft.api.account.host=${minecraft_account_host}".into());
		process
			.jvm_args
			.push("-Dminecraft.api.session.host=${minecraft_session_host}".into());
		process
			.jvm_args
			.push("-Dminecraft.api.services.host=${minecraft_services_host}".into());

		process
			.jvm_args
			.retain(|it| !it.as_str().starts_with("-Djava.library.path"));

		let libraries_base = PathBuf::from("libraries");

		let mut classpath: Vec<_> = classpath
			.into_iter()
			.map(|it| libraries_base.join(it))
			.collect();

		let version_base = PathBuf::from("versions").join(&manifest.id);

		classpath.push(version_base.join(format!("{}.jar", &manifest.id)));

		process
			.vars
			.insert("classpath".into(), classpath.join_classpath()?);

		process.vars.insert("version_name".into(), manifest.id);
		process
			.vars
			.insert("version_type".into(), manifest.version_type);
		process
			.vars
			.insert("assets_index_name".into(), manifest.assets);
		process.vars.insert("assets_root".into(), "assets".into());
		process.vars.insert(
			"natives_directory".into(),
			version_base
				.join("natives")
				.to_str()
				.ok_or(Error::InvalidUtf8Path)?
				.to_owned(),
		);

		Ok(process)
	}
}

#[cfg(feature = "install")]
impl crate::install::Install for Mojang {
	type Error = Error;

	async fn install(self) -> Result<Vec<Action>, Self::Error> {
		let mut actions = Vec::with_capacity(4096);

		let versions_dir = PathBuf::from("versions");
		let assets_dir = PathBuf::from("assets");
		let libraries_dir = PathBuf::from("libraries");
		let jre_dir = PathBuf::from("jre");

		let manifest = match self.0 {
			Manifest::Root(it) => it,
			Manifest::Inherited(it) => {
				let id = it.id.clone();
				let manifest = Self::fetch_manifest(&VersionOptions { mcv: id.clone() }).await?;
				let root = manifest
					.into_root()
					.expect("Mojang manifest unexpectedly inherited");

				actions.push(Action::Write(WriteAction {
					path: versions_dir.join(id),
					content: serde_json::to_string(&root)?.into_bytes(),
				}));

				root
			}
		};

		let jre_component_dir = jre_dir.join(&manifest.java_version.component);

		macro_rules! push_action {
			($url:expr, $path:expr) => {
				actions.push(Action::Download(DownloadAction {
					url: $url,
					path: $path,
					known_size: None,
					known_sha: None,
					ignore_integrity: false,
				}))
			};
			($url:expr, $path:expr, $size:expr) => {
				actions.push(Action::Download(DownloadAction {
					url: $url,
					path: $path,
					known_size: Some($size),
					known_sha: None,
					ignore_integrity: false,
				}))
			};
			($url:expr, $path:expr, $size:expr, $sha:expr) => {
				actions.push(Action::Download(DownloadAction {
					url: $url,
					path: $path,
					known_size: Some($size),
					known_sha: Some($sha),
					ignore_integrity: false,
				}))
			};
		}

		let version_dir = versions_dir.join(&manifest.id);

		push_action!(
			manifest.downloads.client.url,
			version_dir.join(format!("{}.jar", &manifest.id)),
			manifest.downloads.client.size,
			manifest.downloads.client.sha1
		);

		for lib in manifest.libraries {
			use Library::*;

			match lib {
				Custom { ref url, .. } => {
					// TODO: Drop unwrap
					let path = lib.derive_path().unwrap();
					let url = url.join(path.to_str().ok_or(Error::InvalidUtf8Path)?)?;
					push_action!(url, libraries_dir.join(path))
				}
				Native {
					rules,
					mut downloads,
					natives,
					..
				} => {
					push_action!(
						downloads.artifact.url,
						libraries_dir.join(downloads.artifact.path),
						downloads.artifact.size,
						downloads.artifact.sha1
					);

					if !rules.iter().all(Rule::unwrap) {
						continue;
					}

					let classifier = natives.get_classifier_name()?;

					let artifact = downloads
						.classifiers
						.remove(classifier)
						.ok_or(Error::Malformed("Inappropriate native classifier".into()))?;

					push_action!(
						artifact.url,
						libraries_dir.join(artifact.path),
						artifact.size,
						artifact.sha1
					);
				}
				Seminative {
					rules, downloads, ..
				} => {
					if !rules.iter().all(Rule::unwrap) {
						continue;
					}

					push_action!(
						downloads.artifact.url,
						libraries_dir.join(downloads.artifact.path),
						downloads.artifact.size,
						downloads.artifact.sha1
					);
				}
				Default { downloads, .. } => {
					push_action!(
						downloads.artifact.url,
						libraries_dir.join(downloads.artifact.path),
						downloads.artifact.size,
						downloads.artifact.sha1
					);
				}
			}
		}

		push_action!(
			manifest.asset_index.url.clone(),
			assets_dir
				.join("indexes")
				.join(format!("{}.json", manifest.asset_index.id)),
			manifest.asset_index.size,
			manifest.asset_index.sha1
		);

		let asset_index: AssetIndex = reqwest::get(manifest.asset_index.url).await?.json().await?;

		let assets_objects_dir = assets_dir.join("objects");

		for it in asset_index.objects.into_values() {
			let path = format!("{}/{}", &it.hash[..2], it.hash);

			push_action!(
				Url::parse(&format!(
					"https://resources.download.minecraft.net/{}",
					path
				))?,
				assets_objects_dir.join(path),
				it.size,
				it.hash
			);
		}

		let jre_manifest = get_jre_components().await?;
		let jre_target = jre_manifest.get_components_for_current_target()?;
		let jre_component = jre_target.get_component(&JreComponentType::from_str(
			&manifest.java_version.component,
		))?;

		let jre_manifest: JreManifest = reqwest::get(jre_component.manifest.url.clone())
			.await?
			.json()
			.await?;

		for (path, entry) in jre_manifest.files {
			if let JreEntry::File(it) = entry {
				let file = it.downloads.raw;

				push_action!(file.url, jre_component_dir.join(path), file.size, file.sha1)
			}
		}

		Ok(actions)
	}
}
