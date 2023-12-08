1use {
	crate::Error,
	std::{
		io::{
			BufRead,
			BufReader,
		},
		path::PathBuf,
		process::{
			Command,
			Stdio,
		},
	},
	tokio::{
		sync::{
			mpsc,
			mpsc::Receiver,
		},
		task::JoinHandle,
	},
};

#[derive(Debug, Clone)]
pub struct LaunchCommand {
	pub bin: PathBuf,
	pub jvm_args: Vec<String>,
	pub main_class: String,
	pub game_args: Vec<String>,
}

impl LaunchCommand {
	fn launch(self) -> Result<(JoinHandle<()>, JoinHandle<()>, Receiver<String>), Error> {
		let mut cmd: Command = self.into();

		cmd.stdout(Stdio::piped());
		cmd.stderr(Stdio::piped());

		let process = cmd.spawn()?;

		let (mut tx, rx) = mpsc::channel::<String>(64);

		let stdout_task = tokio::spawn({
			let pipe = process.stdout.unwrap();
			let mut buf = BufReader::new(pipe);

			async move {
				loop {
					let mut out = String::new();

					if let Ok(bytes) = buf.read_line(&mut out) {
						let _ = tx.send(out).await;

						if bytes == 0 && out.is_empty() {
							break;
						}
					}
				}
			}
		});

		let stderr_task = tokio::spawn({
			let pipe = process.stderr.unwrap();
			let mut buf = BufReader::new(pipe);

			async move {
				loop {
					let mut out = String::new();

					if let Ok(bytes) = buf.read_line(&mut out) {
						let _ = tx.send(out).await;

						if bytes == 0 && out.is_empty() {
							break;
						}
					}
				}
			}
		});

		Ok((stdout_task, stderr_task, rx))
	}
}

impl From<LaunchCommand> for Command {
	fn from(value: LaunchCommand) -> Self {
		let mut cmd = Command::new(value.bin);

		cmd.args(value.jvm_args);
		cmd.arg(value.main_class);
		cmd.args(value.game_args);

		cmd
	}
}
