use {
  serde::{
    Deserialize,
    Serialize,
  },
  std::{
    collections::HashMap,
    path::Path,
  },
  tokio::fs,
};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileEntry {
  pub last_version_id: String,
  pub icon: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Profile {
  pub profiles: HashMap<String, ProfileEntry>,
}

pub async fn read_profile(root: &Path) -> Result<Profile, std::io::Error> {
  let contents = fs::read_to_string(root.join("profile.json")).await?;

  Ok(serde_json::from_str(contents.as_str())?)
}
