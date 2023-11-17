use {
	common::{
		jre::{
			all::JavaRuntime,
			manifest::Manifest as JreManifest,
    },
		manifest::RootManifest,
  },
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
  pub id: String,
  #[serde(rename = "type")]
  pub version_type: String,
  pub url: Url,
  pub sha1: String,
}

#[derive(Debug, Deserialize)]
pub struct Latest {
  pub release: String,
  pub snapshot: String,
}

#[derive(Debug, Deserialize)]
pub struct Versions {
  pub latest: Latest,
  pub versions: Vec<Version>,
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

pub async fn get_manifest(sha: &str, id: &str) -> Result<RootManifest, reqwest::Error> {
  reqwest::get(
    PISTON_META_BASE_URL
      .join(&format!("v1/packages/{sha}/{id}.json"))
      .unwrap(),
  )
  .await?
  .json()
  .await
}

pub async fn get_jre_components() -> Result<JavaRuntime, reqwest::Error> {
  reqwest::get("https://launchermeta.mojang.com/v1/products/java-runtime/2ec0cc96c44e5a76b9c8b7c39df7210883d12871/all.json")
  .await?
  .json()
  .await
}

pub async fn get_jre_manifest(sha: &str) -> Result<JreManifest, reqwest::Error> {
  reqwest::get(
    PISTON_META_BASE_URL
      .join(&format!("v1/packages/{sha}/manifest.json"))
      .unwrap(),
  )
  .await?
  .json()
  .await
}
