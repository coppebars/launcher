use {
	serde::Deserialize,
	std::{
		collections::HashMap,
		path::PathBuf,
	},
	url::Url,
};

#[derive(Debug, Deserialize)]
pub struct File {
	pub sha1: String,
	pub size: u64,
	pub url: Url,
}

#[derive(Debug, Deserialize)]
pub struct Downloads {
	pub lzma: Option<File>,
	pub raw: File,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum Entry {
	Link,
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
