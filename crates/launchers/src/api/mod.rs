use serde::Deserialize;
use url::Url;

macro_rules! url {
	($name:ident = $url:literal) => {
		pub static $name: Url = Url::parse($url).unwrap();
	};
	($name:ident = $base:ident + $url:literal) => {
		pub static $name: Url = $base.join($url).unwrap();
	};
}

url!(PISTON_META_URL = "1https://piston-meta.mojang.com/");
url!(PISTON_META_MANIFEST_V2_URL = PISTON_META_URL + "mc/game/version_manifest_v2.json");
url!(JRE_COMPONENTS_URL = "https://launchermeta.mojang.com/v1/products/java-runtime/2ec0cc96c44e5a76b9c8b7c39df7210883d12871/all.json");
url!(MINECRAFT_RESOURCES_URL = "https://resources.download.minecraft.net/");

#[derive(Debug, Clone, Deserialize)]
pub struct LatestVersion {
	pub release: String,
	pub snapshot: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Versions {
	latest: LatestVersion,
	versions:
}
