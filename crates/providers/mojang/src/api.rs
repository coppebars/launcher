use {
  common::manifest::Manifest,
  once_cell::sync::Lazy,
  serde::Deserialize,
  url::Url,
};

pub static PISTON_META_BASE_URL: Lazy<Url> =
  Lazy::new(|| Url::parse("https://piston-meta.mojang.com/").unwrap());

pub static MINECRAFT_RESOURCES_BASE_URL: Lazy<Url> =
  Lazy::new(|| Url::parse("https://resources.download.minecraft.net").unwrap());

#[derive(Debug, Deserialize)]
pub struct Version {
  id: String,
  #[serde(rename = "type")]
  version_type: String,
  url: Url,
}

#[derive(Debug, Deserialize)]
pub struct Latest {
  release: String,
  snapshot: String,
}

#[derive(Debug, Deserialize)]
pub struct Versions {
  latest: Latest,
  versions: Vec<Version>,
}

pub async fn get_versions_manifest() -> Result<Box<Versions>, reqwest::Error> {
  reqwest::get(
    PISTON_META_BASE_URL
      .join("mc/game/version_manifest_v2.json")
      .unwrap(),
  )
  .await?
  .json()
  .await
}

pub async fn get_manifest(sha: &str, id: &str) -> Result<Manifest, reqwest::Error> {
  reqwest::get(
    PISTON_META_BASE_URL
      .join(&format!("v1/packages/{sha}/{id}.json"))
      .unwrap(),
  )
  .await?
  .json()
  .await
}
