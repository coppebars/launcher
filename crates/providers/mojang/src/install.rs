use {
  common::manifest::{
    Artifact,
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
use common::manifest::AssetIndex;
use crate::api::MINECRAFT_RESOURCES_BASE_URL;

pub enum Kind {
  Lib,
  Native,
  Asset,
  Client,
}

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

trait InstallManifest {
  fn unwrap_libs(self) -> Result<Vec<Item>, InstallError>;
  fn unwrap_natives(self) -> Result<Vec<Item>, InstallError>;
}

trait InstallAssets {
	fn unwrap_assets(self) -> Vec<Item>;
}

impl InstallManifest for RootManifest {
  fn unwrap_libs(self) -> Result<Vec<Item>, InstallError> {
    self
      .libraries
      .into_iter()
      .filter(|it| match it {
        Library::Custom { .. } => true,
        Library::Seminative { rules, .. } => rules.iter().all(Rule::unwrap),
        Library::Default { .. } => true,
        _ => false,
      })
      .map(|it| match it {
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
            .map_err(|_| InstallError::InvalidManifest(sub.into()))?;

          Ok(Item {
            kind: Kind::Lib,
            url,
            path,
            known_size: None,
            known_sha: None,
          })
        }
        Library::Seminative { downloads, .. } => Ok(downloads.artifact.into()),
        Library::Default { downloads, .. } => Ok(downloads.artifact.into()),
        _ => unreachable!(),
      })
      .collect()
  }

  fn unwrap_natives(self) -> Result<Vec<Item>, InstallError> {
    let intermediate: Result<Vec<_>, _> = self
      .libraries
      .into_iter()
      .filter(|it| matches!(it, Library::Native { .. }))
      .map(|it| -> Result<Vec<Item>, InstallError>  {
				match it {
					Library::Native {
						rules,
						downloads,
						natives,
						mut classifiers,
						..
					} => {
						let mut base = Vec::with_capacity(2);
						base.push(Item {
							kind: Kind::Lib,
							url: downloads.artifact.url,
							path: downloads.artifact.path,
							known_size: Some(downloads.artifact.size),
							known_sha: Some(downloads.artifact.sha1),
						});

						if rules.iter().all(Rule::unwrap) {
							#[cfg(target_os = "windows")]
								let classifier = natives.get(&Os::Windows);
							#[cfg(target_os = "macos")]
								let classifier = natives.get(&Os::Osx);
							#[cfg(target_os = "linux")]
								let classifier = natives
								.get(&Os::Linux)
								.ok_or(InstallError::InvalidManifest(
									"Inappropriate native classifier".into(),
								))?;
							#[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
							panic!("Unsupported os");

							let Artifact { url, path, size, sha1 } = classifiers.remove(classifier).ok_or(InstallError::InvalidManifest(
								"Inappropriate native classifier".into(),
							))?;

							base.push(Item {
								kind: Kind::Native,
								url,
								path,
								known_size: Some(size),
								known_sha: Some(sha1),
							});

							Ok(base)
						} else {
							Ok(base)
						}
					}
					_ => unreachable!(),
				}
			})
			.collect();

		Ok(intermediate?.into_iter().flatten().collect::<Vec<_>>())
  }
}


impl InstallAssets for AssetIndex {
	fn unwrap_assets(self) -> Vec<Item> {
		self.objects.into_values().map(|it| {
			let path = format!("{}/{}", &it.hash[..2], it.hash);

			Item {
				kind: Kind::Asset,
				url: MINECRAFT_RESOURCES_BASE_URL.join(&path).unwrap(),
				path: path.into(),
				known_size: Some(it.size),
				known_sha: Some(it.hash),
			}
		}).collect()
	}
}
