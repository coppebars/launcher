use std::io::ErrorKind;
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

pub async fn read_profile(root: &Path) -> Result<Profile, std::io::Error> {
  let contents = fs::read_to_string(root.join("profile.json")).await?;

  Ok(serde_json::from_str(contents.as_str())?)
}

pub async fn create_empty_profile(root: &Path) -> Result<(), std::io::Error> {
  let mut file = fs::File::create(root).await?;

  file.write_all(br#"{"profiles":{}}"#).await?;

  Ok(())
}
