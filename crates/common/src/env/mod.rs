use {
  serde::{
    Deserialize,
    Serialize,
  },
  std::path::PathBuf,
};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Env {
  pub jre_bin: PathBuf,
  pub versions_dir: PathBuf,
  pub lib_dir: PathBuf,
  pub natives_dir: PathBuf,
  pub assets_dir: PathBuf,
}
