use super::prelude::*;

pub mod versions {
	use super::*;

	#[derive(Debug, Deserialize)]
	pub struct Latest {
		pub release: String,
		pub snapshot: String,
	}

	#[derive(Debug, Deserialize)]
	pub struct Version {
		pub id: String,
		#[serde(rename = "type")]
		pub version_type: String,
		pub url: Url,
		pub sha1: String,
	}

	#[derive(Debug, Deserialize)]
	pub struct Response {
		pub latest: Latest,
		pub versions: Vec<Version>,
	}

	impl Response {
		pub fn iter(&self) -> Iter<'_, Version> {
			self.versions.iter()
		}
	}

	impl IntoIterator for Response {
		type IntoIter = IntoIter<Version>;
		type Item = Version;

		fn into_iter(self) -> Self::IntoIter {
			self.versions.into_iter()
		}
	}

	pub const URL: Url = url!("https://piston-meta.mojang.com/mc/game/version_manifest_v2.json");

	pub async fn get() -> Result<Response, reqwest::Error> {
		reqwest::get(URL).await?.json().await
	}
}
