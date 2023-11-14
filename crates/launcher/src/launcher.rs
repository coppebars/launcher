use std::fs::canonicalize;
use {
  std::{
    collections::HashMap,
    iter,
    path::PathBuf,
    process::Command,
  },
  thiserror::Error,
};

#[derive(Debug)]
pub struct Launcher {
  pub bin: PathBuf,
  pub id: String,
  pub classpath: Vec<PathBuf>,
  pub asset_index_name: String,
  pub version_type: String,
  pub jvm_args: Vec<String>,
  pub game_args: Vec<String>,
  pub main_class: String,
  pub root_dir: PathBuf,
  pub game_dir: PathBuf,
  pub username: String,
  pub uuid: String,
  pub access_token: String,
  pub width: u32,
  pub height: u32,
  pub fullscreen: bool,
}

impl Default for Launcher {
  fn default() -> Self {
    let home_dir = directories::BaseDirs::new()
      .unwrap()
      .home_dir()
      .join(".minecraft");

    Self {
      bin: "java".into(),
      id: "".to_string(),
      width: 1280,
      height: 720,
      root_dir: home_dir.clone(),
      game_dir: home_dir.clone(),
      username: "Player".into(),
      uuid: "00000000-0000-0000-0000-000000000000".into(),
      fullscreen: false,
      access_token: "local".into(),
      classpath: Vec::new(),
			asset_index_name: String::new(),
			version_type: "release".into(),
			jvm_args: Vec::new(),
      game_args: Vec::new(),
      main_class: String::new(),
    }
  }
}

#[derive(Debug, Error)]
pub enum LaunchError {
  #[error("Invalid path")]
  InvalidPath,

	#[error(transparent)]
	Io(#[from] std::io::Error)
}

impl Launcher {
  pub fn launch(&self) -> Result<Command, LaunchError> {
    let lib_dir = self.root_dir.join("libraries");
    let nat_dir = self.root_dir.join("versions").join(&self.id).join("natives");
    let assets_dir = self.root_dir.join("assets");

    let classpath = self.classpath.iter().map(|it| {
			Ok(canonicalize(lib_dir.join(it))?.to_str().ok_or(LaunchError::InvalidPath)?.to_owned())
		}).collect::<Result<Vec<_>, LaunchError>>()?;

    let mut vars = HashMap::<&str, &str>::new();

		let var_game_dir = canonicalize(&self.game_dir)?;
		let var_assets_dir = canonicalize(assets_dir)?;
		let var_nat_dir = canonicalize(nat_dir)?;

    vars.insert("${auth_player_name}", &self.username);
    vars.insert("${version_name}", &self.id);
    vars.insert(
      "${game_directory}",
			var_game_dir.to_str().ok_or(LaunchError::InvalidPath)?,
    );
    vars.insert(
      "${assets_root}",
			var_assets_dir.to_str().ok_or(LaunchError::InvalidPath)?,
    );
		vars.insert(
      "${natives_directory}",
			var_nat_dir.to_str().ok_or(LaunchError::InvalidPath)?,
    );

		let width = self.width.to_string();
		let height = self.height.to_string();

    vars.insert("${assets_index_name}", &self.asset_index_name);
    vars.insert("${auth_uuid}", &self.uuid);
    vars.insert("${auth_access_token}", &self.access_token);
    vars.insert("${version_type}", &self.version_type);
    vars.insert("${resolution_width}", &width);
    vars.insert("${resolution_height}", &height);
    vars.insert("${launcher_name}", "coppertiles");
    vars.insert("${launcher_version}", "unknown");

		#[cfg(target_family = "windows")]
		let cp_str = classpath.join(":");
		#[cfg(not(target_family = "windows"))]
		let cp_str = classpath.join(":");

		vars.insert("${classpath}", &cp_str);

    let args: Vec<_> = self
      .jvm_args
      .iter()
      .chain(iter::once(&self.main_class))
      .chain(&self.game_args)
      .map(|arg| {
				let mut arg = arg.to_owned();
				for (key, value) in &vars {
					arg = arg.replace(key, value);
				}
				arg
			})
			.collect();

		let mut command = Command::new(&self.bin);
		command.args(args);
		Ok(command)
  }
}
