use {
  super::api::MINECRAFT_RESOURCES_BASE_URL,
  common::manifest::{
    Artifact,
    AssetIndex,
    Library,
    Os,
    RootManifest,
    Rule,
  },
  regex::Regex,
  serde::Deserialize,
  std::{
    iter,
    path::PathBuf,
  },
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
			path: "client.jar".into(),
			known_size: Some(self.downloads.client.size),
			known_sha: Some(self.downloads.client.sha1),
		});

    for lib in self.libraries {
      match lib {
        Library::Custom { name, url } => {
          let re = Regex::new(r"^([^:]+):([^:]+):(.+)").unwrap();

          let ca = re.captures(&name).ok_or(InstallError::InvalidManifest(
            "Malformed or unsupported artifact name".into(),
          ))?;

          let package = &ca[0];
          let artifact = &ca[1];
          let version = &ca[2];

          let path = PathBuf::from_iter(
            package
              .split('.')
              .chain(iter::once(artifact))
              .chain(iter::once(version)),
          );

          let sub = format!("{}-{}.jar", artifact, version);

          let url = url
            .join(
              path
                .join(&sub)
                .to_str()
                .expect("Failed to parse artifact path"),
            )
            .map_err(|_| InstallError::InvalidManifest(sub))?;

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
    Ok(
      self
        .objects
        .into_values()
        .map(|it| {
          let path = format!("{}/{}", &it.hash[..2], it.hash);

          Item {
            kind: Kind::Asset,
            url: MINECRAFT_RESOURCES_BASE_URL.join(&path).unwrap(),
            path: path.into(),
            known_size: Some(it.size),
            known_sha: Some(it.hash),
          }
        })
        .collect(),
    )
  }
}
