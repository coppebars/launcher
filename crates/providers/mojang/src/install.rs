use {
	common::{
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
		get_versions_manifest,
		MINECRAFT_RESOURCES_BASE_URL,
  },
	serde::Deserialize,
	std::path::PathBuf,
	thiserror::Error,
	url::Url,
};

#[derive(Debug)]
pub enum Kind {
  Lib,
  Native,
  Asset,
  Version,
}

#[derive(Debug)]
pub struct Item {
  pub kind: Kind,
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
}

pub trait Install {
  fn into_items(self) -> Result<Vec<Item>, InstallError>;
}

impl Install for RootManifest {
  fn into_items(self) -> Result<Vec<Item>, InstallError> {
    let mut items: Vec<Item> = Vec::new();

    items.push(Item {
      kind: Kind::Version,
      url: self.downloads.client.url,
      path: PathBuf::from(&self.id).join("client.jar"),
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
            kind: Kind::Lib,
            url,
            path,
            known_size: None,
            known_sha: None,
          });
        }
        Library::Native {
          rules,
          downloads,
          natives,
          mut classifiers,
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
          } = classifiers
            .remove(classifier)
            .ok_or(InstallError::InvalidManifest(
              "Inappropriate native classifier".into(),
            ))?;

          items.push(Item {
            kind: Kind::Native,
            url,
            path,
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

pub async fn get_items(id: &str) -> Result<Vec<Item>, Box<dyn std::error::Error>> {
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

  let mut items = Vec::with_capacity(4096);

  items.push(Item {
    kind: Kind::Version,
    url: version.url,
    path: PathBuf::from(id).join("version.json"),
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

  items.extend(manifest.into_items()?);
  items.extend(asset_index.into_items()?);

  Ok(items)
}
