use serde::{
	Deserialize,
	Serialize,
};

#[derive(Debug, Deserialize)]
pub struct ManifestPart {
	pub id: String,
}

#[derive(Debug, Serialize)]
pub struct VersionOverview {
	pub id: String,
	pub icon: Option<String>,
}
