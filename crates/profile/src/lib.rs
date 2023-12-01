use {
	serde::{
		Deserialize,
		Serialize,
	},
	std::{
		collections::HashMap,
		path::Path,
	},
	tokio::{
		fs,
		io::AsyncWriteExt,
	},
};

const LAUNCHER_PROFILE: &str = "launcher_profiles.json";

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileEntry {
	pub last_version_id: String,
	pub icon: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Profile {
	pub profiles: HashMap<String, ProfileEntry>,
}

impl Profile {
	pub async fn read_from_canonical_root(root: &Path) -> Result<Self, std::io::Error> {
		let contents = fs::read_to_string(root.join(LAUNCHER_PROFILE)).await?;

		Ok(serde_json::from_str(contents.as_str())?)
	}

	pub async fn new_empty_profile() -> Self {
		Default::default()
	}

	pub async fn insert(&mut self, entry: ProfileEntry) -> Option<ProfileEntry> {
		self.profiles.insert(entry.last_version_id.clone(), entry)
	}

	pub async fn remove(&mut self, id: &str) -> Option<ProfileEntry> {
		self.profiles.remove(id)
	}

	pub async fn save_to_canonical_root(&self, root: &Path) -> Result<(), std::io::Error> {
		let mut file = fs::File::create(root.join(LAUNCHER_PROFILE)).await?;

		let serialized = serde_json::to_string(self)?;

		file.write_all(serialized.as_bytes()).await?;

		Ok(())
	}
}
