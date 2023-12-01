use {
	crate::launch::utils::setup_permissions,
	std::{
		collections::HashMap,
		path::PathBuf,
		process::Command,
	},
};

#[derive(Debug, Default)]
pub struct ProcessLauncher {
	pub cwd: PathBuf,
	pub bin: PathBuf,
	pub jvm_args: Vec<String>,
	pub main_class: String,
	pub game_args: Vec<String>,
	pub vars: HashMap<String, String>,
	pub alloc: Option<(u32, u32)>,
}

impl ProcessLauncher {
	fn set_vars(&self, target: &str) -> String {
		let mut target = target.to_owned();

		for (key, value) in &self.vars {
			target = target.replace(&format!("${{{key}}}"), value);
		}

		target
	}

	pub fn into_command(self) -> Command {
		let full_bin_path = self.cwd.join(&self.bin);

		setup_permissions(&full_bin_path);

		let mut cmd = Command::new(full_bin_path);

		cmd.current_dir(&self.cwd);

		let mut jvm_alloc_args = Vec::with_capacity(2);

		if let Some((xms, xmx)) = self.alloc {
			jvm_alloc_args.push(format!("-Xms{}M", xms));
			jvm_alloc_args.push(format!("-Xmx{}M", xmx));
		}

		cmd.args(
			jvm_alloc_args
				.iter()
				.chain(&self.jvm_args)
				.map(|it| self.set_vars(it)),
		);
		cmd.arg(&self.main_class);
		cmd.args(self.game_args.iter().map(|it| self.set_vars(it)));

		cmd
	}

	pub fn launch_blocking(self) {
		let mut command = self.into_command();

		command.spawn().unwrap().wait().unwrap();
	}
}

impl From<ProcessLauncher> for Command {
	fn from(value: ProcessLauncher) -> Self {
		value.into_command()
	}
}
