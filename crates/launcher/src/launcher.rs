use {
  common::env::Env,
  std::{
    path::PathBuf,
    process::Command,
  },
};

pub struct Launcher {
  pub id: String,
  pub main: String,
  pub libs: Vec<PathBuf>,
  pub jvm_args: Vec<String>,
  pub game_args: Vec<String>,
}

impl Launcher {
  pub fn launch(&self, env: Env) -> Command {
    let mut command = Command::new(env.jre_bin);
    command.args(&self.jvm_args);
    command.args(&self.jvm_args);
    command.arg(&self.main);
    command.args(&self.game_args);

    command
  }
}
