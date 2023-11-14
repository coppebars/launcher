use {
	common::manifest::RootManifest,
	launcher::Launcher,
	std::{
		fs::File,
		io::Read,
	},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut json = String::new();
  File::open("./minecraft/versions/1.20.1/version.json")
    .unwrap()
    .read_to_string(&mut json)
    .unwrap();

  let manifest: RootManifest = serde_json::from_str(&json).unwrap();

  let mut launcher: Launcher = manifest.try_into()?;

  launcher.game_dir = "./minecraft/game".into();
  launcher.root_dir = "./minecraft".into();

  let mut command = launcher.launch()?;

  command.spawn()?.wait()?;

  Ok(())
}
