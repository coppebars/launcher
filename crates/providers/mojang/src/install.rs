use {
	crate::api::{
		get_jre_components,
		get_jre_manifest,
		MINECRAFT_RESOURCES_BASE_URL,
	},
	common::{
		jre::{
			all::{
				ComponentType,
				Target,
			},
			manifest::{
				Entry,
				Manifest as JreManifest,
			},
		},
		libutils::libname_to_path,
		manifest::{
			Artifact,
			AssetIndex,
			Library,
			Os,
			RootManifest,
			Rule,
		},
		tree::CanonicalKind,
	},
	reqwest::Error,
	serde::Deserialize,
	std::path::PathBuf,
	thiserror::Error,
	url::Url,
};

#[derive(Debug)]
pub struct Item {
	pub kind: CanonicalKind,
	pub url: Url,
	pub path: PathBuf,
	pub known_size: Option<u64>,
	pub known_sha: Option<String>,
}

impl From<Artifact> for Item {
	fn from(
		Artifact {
			url,
			path,
			size,
			sha1,
		}: Artifact,
	) -> Self {
		Self {
			kind: CanonicalKind::Library,
			url,
			path,
			known_size: Some(size),
			known_sha: Some(sha1),
		}
	}
}

#[derive(Debug, Error, Deserialize)]
pub enum InstallError {
	#[error("Malformed or unsupported manifest: {0}")]
	InvalidManifest(String),

	#[error("network: {0}")]
	Network(String),

	#[error("unsupported: {0}")]
	Unsupported(String),

	#[error("unexpected")]
	Unexpected(String),
}

impl From<reqwest::Error> for InstallError {
	fn from(value: Error) -> Self {
		Self::Network(value.to_string())
	}
}

pub trait IntoItems {
	fn into_items(self) -> Result<Vec<Item>, InstallError>;
}

impl IntoItems for RootManifest {
	fn into_items(self) -> Result<Vec<Item>, InstallError> {
		let mut items: Vec<Item> = Vec::new();

		let version_path = PathBuf::from(&self.id);

		items.push(Item {
			kind: CanonicalKind::Version,
			url: self.downloads.client.url,
			path: version_path.join(format!("{}.jar", &self.id)),
			known_size: Some(self.downloads.client.size),
			known_sha: Some(self.downloads.client.sha1),
		});

		for lib in self.libraries {
			match lib {
				Library::Custom { name, url } => {
					let path = libname_to_path(&name)
						.ok_or(InstallError::InvalidManifest("Invalid lib name".into()))?;

					let url = url
						.join(path.to_str().expect("Failed to parse artifact path"))
						.map_err(|_| InstallError::InvalidManifest("Failed to parse lib url".into()))?;

					items.push(Item {
						kind: CanonicalKind::Library,
						url,
						path,
						known_size: None,
						known_sha: None,
					});
				}
				Library::Native {
					rules,
					mut downloads,
					natives,
					..
				} => {
					items.push(Item {
						kind: CanonicalKind::Library,
						url: downloads.artifact.url,
						path: downloads.artifact.path,
						known_size: Some(downloads.artifact.size),
						known_sha: Some(downloads.artifact.sha1),
					});

					if !rules.iter().all(Rule::unwrap) {
						continue;
					}

					#[cfg(target_os = "windows")]
					let classifier = natives.get(&Os::Windows);
					#[cfg(target_os = "macos")]
					let classifier = natives.get(&Os::Osx);
					#[cfg(target_os = "linux")]
					let classifier = natives.get(&Os::Linux);
					#[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
					panic!("Unsupported os");

					let classifier = classifier.ok_or(InstallError::InvalidManifest(
						"Inappropriate native classifier".into(),
					))?;

					let artifact =
						downloads
							.classifiers
							.remove(classifier)
							.ok_or(InstallError::InvalidManifest(
								"Inappropriate native classifier".into(),
							))?;

					items.push(artifact.into());
				}
				Library::Seminative {
					rules, downloads, ..
				} => {
					if !rules.iter().all(Rule::unwrap) {
						continue;
					}

					items.push(downloads.artifact.into());
				}
				Library::Default { downloads, .. } => {
					items.push(downloads.artifact.into());
				}
			}
		}

		Ok(items)
	}
}

impl IntoItems for AssetIndex {
	fn into_items(self) -> Result<Vec<Item>, InstallError> {
		let mut items: Vec<Item> = Vec::with_capacity(2048);

		for it in self.objects.into_values() {
			let path = format!("{}/{}", &it.hash[..2], it.hash);

			items.push(Item {
				kind: CanonicalKind::AssetsObject,
				url: MINECRAFT_RESOURCES_BASE_URL.join(&path).unwrap(),
				path: path.into(),
				known_size: Some(it.size),
				known_sha: Some(it.hash),
			});
		}

		Ok(items)
	}
}

impl IntoItems for JreManifest {
	fn into_items(self) -> Result<Vec<Item>, InstallError> {
		Ok(
			self
				.files
				.into_iter()
				.filter_map(|(path, entry)| match entry {
					Entry::File { downloads, .. } => {
						let file = downloads.raw;

						Some(Item {
							kind: CanonicalKind::Jre,
							path,
							url: file.url,
							known_size: Some(file.size),
							known_sha: Some(file.sha1),
						})
					}
					_ => None,
				})
				.collect(),
		)
	}
}

pub async fn get_items(manifest: RootManifest) -> Result<Vec<Item>, InstallError> {
	let asset_index = reqwest::get(manifest.asset_index.url.clone())
		.await?
		.json::<AssetIndex>()
		.await?;

	let jre_manifest = get_jre_components().await?;
	let jre_target = jre_manifest
		.get(
			#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
			&Target::Linux,
			#[cfg(all(target_os = "linux", target_arch = "x86"))]
			&Target::LinuxI386,
			#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
			&Target::WindowsX64,
			#[cfg(all(target_os = "windows", target_arch = "x86"))]
			&Target::WindowsX86,
			#[cfg(all(target_os = "windows", target_arch = "aarch64"))]
			&Target::WindowsArm64,
			#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
			&Target::Macos,
			#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
			&Target::MacosArm64,
		)
		.ok_or(InstallError::Unsupported("Unsupported platform".into()))?;

	let jre_component = jre_target
		.get(
			&ComponentType::from_str(&manifest.java_version.component)
				.ok_or(InstallError::Unexpected("Unknown jre component".into()))?,
		)
		.ok_or(InstallError::Unexpected("No such jre component".into()))?
		.get(0)
		.ok_or(InstallError::Unsupported(
			"Jre is not supported for current platform".into(),
		))?;

	let jre_manifest = get_jre_manifest(&jre_component.manifest.sha1).await?;

	let mut items = Vec::with_capacity(8192);

	items.push(Item {
		kind: CanonicalKind::AssetsIndex,
		url: manifest.asset_index.url.clone(),
		path: format!("{}.json", &manifest.assets).into(),
		known_size: None,
		known_sha: Some(manifest.asset_index.sha1.clone()),
	});

	items.extend(manifest.into_items()?);
	items.extend(asset_index.into_items()?);
	items.extend(jre_manifest.into_items()?);

	Ok(items)
}
