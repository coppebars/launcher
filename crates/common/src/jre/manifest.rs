use {
  serde::Deserialize,
  std::{
    collections::HashMap,
    path::PathBuf,
  },
};

#[derive(Debug, Deserialize)]
pub struct File {
  pub sha1: String,
  pub size: u32,
  pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Downloads {
  pub lzma: File,
  pub raw: File,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum Entry {
  Directory,
  File {
    executable: bool,
    downloads: Downloads,
  },
}

pub type Files = HashMap<PathBuf, Entry>;

#[derive(Debug, Deserialize)]
pub struct Manifest {
  pub files: Files,
}
