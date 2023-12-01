use {
	std::path::PathBuf,
	url::Url,
};

#[derive(Debug, Clone)]
pub struct DownloadAction {
	pub path: PathBuf,
	pub url: Url,
	pub known_size: Option<u64>,
	pub known_sha: Option<String>,
	pub ignore_integrity: bool,
}

#[derive(Debug, Clone)]
pub struct WriteAction {
	pub path: PathBuf,
	pub content: Vec<u8>,
}

#[derive(Debug, Clone)]
pub enum Action {
	Download(DownloadAction),
	Write(WriteAction)
}
