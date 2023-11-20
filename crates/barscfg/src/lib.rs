use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Cfg {
	pub root: PathBuf,
}

impl Default for Cfg {
	fn default() -> Self {
		Self {
			root: directories::BaseDirs::new()
				.unwrap()
				.home_dir()
				.join(".coppebars"),
		}
	}
}
