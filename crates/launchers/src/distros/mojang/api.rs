use {
	crate::specs::jre::JavaRuntime,
	serde::Deserialize,
	url::Url,
};

#[derive(Debug, Deserialize, Clone)]
pub struct Version {
	pub id: String,
	#[serde(rename = "type")]
	pub version_type: String,
	pub url: Url,
	pub sha1: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Latest {
	pub release: String,
	pub snapshot: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Versions {
	pub latest: Latest,
	pub versions: Vec<Version>,
}

pub async fn get_versions() -> Result<Versions, reqwest::Error> {
	reqwest::get("https://piston-meta.mojang.com/mc/game/version_manifest_v2.json")
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
