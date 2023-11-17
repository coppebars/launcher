use {
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
  },
	crate::api::{
		get_jre_components,
		get_jre_manifest,
		get_versions_manifest,
		MINECRAFT_RESOURCES_BASE_URL,
  },
	download::Item as DownloadItem,
	once_cell::sync::Lazy,
	reqwest::Error,
	serde::Deserialize,
	std::{
		collections::HashMap,
		path::{
			Path,
			PathBuf,
    },
  },
	thiserror::Error,
	url::Url,
};

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Kind {
  Lib,
  Asset,
  Version,
  Jre,
}

#[derive(Debug)]
pub struct Item {
  pub kind: Kind,
  pub url: Url,
  pub path: PathBuf,
  pub known_size: Option<u64>,
  pub known_sha: Option<String>,
}

static PATHS: Lazy<HashMap<Kind, PathBuf>> = Lazy::new(|| {
  HashMap::from([
    (Kind::Version, PathBuf::from("versions")),
    (Kind::Lib, PathBuf::from("libraries")),
    (Kind::Asset, PathBuf::from("assets")),
    (Kind::Jre, PathBuf::from("jre")),
  ])
});

impl Item {
  pub fn place(self, root: &Path) -> DownloadItem {
    DownloadItem {
      path: root.join(
        PATHS
          .get(&self.kind)
          .expect("Unknown item kind")
          .join(self.path),
      ),
      url: self.url,
      known_sha: self.known_sha,
      known_size: self.known_size,
    }
  }
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
      kind: Kind::Lib,
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

pub trait Install {
  fn into_items(self) -> Result<Vec<Item>, InstallError>;
}

impl Install for RootManifest {
  fn into_items(self) -> Result<Vec<Item>, InstallError> {
    let mut items: Vec<Item> = Vec::new();

		let version_path = PathBuf::from(&self.id);

    items.push(Item {
      kind: Kind::Version,
      url: self.downloads.client.url,
      path: version_path.join(format!("{}.jar", &self.id)),
      known_size: Some(self.downloads.client.size),
      known_sha: Some(self.downloads.client.sha1),
    });

    let native_path = version_path.join("natives");

    for lib in self.libraries {
      match lib {
        Library::Custom { name, url } => {
          let path = libname_to_path(&name)
            .ok_or(InstallError::InvalidManifest("Invalid lib name".into()))?;

          let url = url
            .join(path.to_str().expect("Failed to parse artifact path"))
            .map_err(|_| InstallError::InvalidManifest("Failed to parse lib url".into()))?;

          items.push(Item {
            kind: Kind::Lib,
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
            kind: Kind::Lib,
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

          let Artifact {
            url,
            path,
            size,
            sha1,
          } = downloads.classifiers
            .remove(classifier)
            .ok_or(InstallError::InvalidManifest(
              "Inappropriate native classifier".into(),
            ))?;

					println!("{:?}", path);
          items.push(Item {
            kind: Kind::Version,
            url,
            path: native_path.join(path.iter().last().unwrap()),
            known_size: Some(size),
            known_sha: Some(sha1),
          });
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

impl Install for AssetIndex {
  fn into_items(self) -> Result<Vec<Item>, InstallError> {
    let base = PathBuf::from("objects");
    let mut items: Vec<Item> = Vec::with_capacity(2048);

    for it in self.objects.into_values() {
      let path = format!("{}/{}", &it.hash[..2], it.hash);

      items.push(Item {
        kind: Kind::Asset,
        url: MINECRAFT_RESOURCES_BASE_URL.join(&path).unwrap(),
        path: base.join(path),
        known_size: Some(it.size),
        known_sha: Some(it.hash),
      });
    }

    Ok(items)
  }
}

trait JreInstall {
	fn into_items(self, jre_component: &Path) -> Result<Vec<Item>, InstallError>;
}

impl JreInstall for JreManifest {
  fn into_items(self, jre_component: &Path) -> Result<Vec<Item>, InstallError> {
    Ok(
      self
        .files
        .into_iter()
        .filter_map(|(path, entry)| match entry {
          Entry::File { downloads, .. } => {
						let file = downloads.raw;

						Some(Item {
							kind: Kind::Jre,
							path: jre_component.join(path),
							url: file.url,
							known_size: Some(file.size),
							known_sha: Some(file.sha1),
						})
					},
          _ => None,
        })
        .collect(),
    )
  }
}

pub async fn get_items(id: &str) -> Result<Vec<Item>, InstallError> {
  let versions = get_versions_manifest().await?;
  let version = versions
    .versions
    .into_iter()
    .find(|it| it.id == id)
    .expect("Unknown id");

  let manifest: RootManifest = reqwest::get(version.url.clone()).await?.json().await?;
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
      &Target::Macos,
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
    kind: Kind::Version,
    url: version.url,
    path: PathBuf::from(id).join(format!("{id}.json")),
    known_size: None,
    known_sha: Some(version.sha1),
  });

  items.push(Item {
    kind: Kind::Asset,
    url: manifest.asset_index.url.clone(),
    path: PathBuf::from("indexes").join(format!("{}.json", &manifest.assets)),
    known_size: None,
    known_sha: Some(manifest.asset_index.sha1.clone()),
  });

	let jre_component = manifest.java_version.component.clone();

  items.extend(manifest.into_items()?);
  items.extend(asset_index.into_items()?);
  items.extend(jre_manifest.into_items(Path::new(&jre_component))?);

  Ok(items)
}
