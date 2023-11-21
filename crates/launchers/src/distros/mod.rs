use {
	std::path::PathBuf,
	url::Url,
};

#[cfg(feature = "mojang")]
pub mod mojang;

#[derive(Debug, Clone)]
pub struct PrepareRemoteFileAction {
	pub path: PathBuf,
	pub url: Url,
	pub known_size: Option<u64>,
	pub known_sha: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PrepareWriteAction {
	pub path: PathBuf,
	pub content: Vec<u8>,
}

#[derive(Debug, Clone)]
pub enum PrepareAction {
	RemoteFile(PrepareRemoteFileAction),
	Write(PrepareWriteAction),
}
