use {
	launchers::{
		distros::mojang::Mojang,
		misc::DownloadEvent,
	},
	std::{
		collections::HashMap,
		io::{
			BufRead,
			BufReader,
		},
		path::{
			Path,
			PathBuf,
		},
		process::Stdio,
		sync::Arc,
		time::{
			Duration,
			SystemTime,
		},
	},
	tauri::{
		Manager,
		Window,
	},
	tokio::sync::mpsc,
	tokio_util::sync::CancellationToken,
};

#[tauri::command]
pub async fn mojang_prepare(window: Window, id: String, path: PathBuf) -> Result<(), String> {
	let distro = Mojang::try_from_canonical_tree(&path, &id)
		.await
		.map_err(|it| it.to_string())?;

	let actions = distro.prepare().await.map_err(|it| it.to_string())?;

	let (tx, mut rx) = mpsc::channel(1024);
	let token = Arc::new(CancellationToken::new());

	let task_token = token.clone();

	let task = {
		let path = path.clone();

		tokio::spawn(launchers::misc::place_to_canonical_tree(
			path,
			actions,
			Arc::new(tx),
			task_token,
		))
	};

	let now = SystemTime::now();
	let mut next = SystemTime::now().elapsed().unwrap();

	while let Some(msg) = rx.recv().await {
		let elapsed = now.elapsed().unwrap();
		match msg {
			DownloadEvent::Chunk { .. } => {
				if elapsed > next {
					next = elapsed + Duration::from_millis(50);
					window.emit("prepare", &msg).map_err(|it| it.to_string())?;
				}
			}
			_ => {
				window.emit("prepare", &msg).map_err(|it| it.to_string())?;
			}
		}
	}

	task
		.await
		.map_err(|it| it.to_string())?
		.map_err(|it| it.to_string())?;

	Ok(())
}

#[tauri::command]
pub async fn mojang_launch(
	window: Window,
	uid: String,
	id: &str,
	path: &Path,
	vars: HashMap<String, String>,
) -> Result<(), String> {
	let distro = Mojang::try_from_canonical_tree(path, id)
		.await
		.map_err(|it| it.to_string())?;

	let mut launcher = distro.try_into_process().map_err(|it| it.to_string())?;

	launcher.cwd = path.to_owned();
	launcher.vars.extend(vars);

	let mut command = launcher.into_command();

	command.stdout(Stdio::piped());
	command.stderr(Stdio::piped());

	let process = command.spawn().map_err(|it| it.to_string())?;

	let window = Arc::new(window);

	let stdout_task = tokio::spawn({
		let window = window.clone();
		let event_name = format!("log::{uid}");
		let pipe = process.stdout.unwrap();
		let mut buf = BufReader::new(pipe);

		async move {
			loop {
				let mut out = String::new();

				if let Ok(bytes) = buf.read_line(&mut out) {
					let _ = window.emit(&event_name, &out);

					if bytes == 0 && out.is_empty() {
						break;
					}
				}
			}
		}
	});

	let stderr_task = tokio::spawn({
		let window = window.clone();
		let event_name = format!("log::{uid}");
		let pipe = process.stderr.unwrap();
		let mut buf = BufReader::new(pipe);

		async move {
			loop {
				let mut out = String::new();

				if let Ok(bytes) = buf.read_line(&mut out) {
					let _ = window.emit(&event_name, &out);

					if bytes == 0 && out.is_empty() {
						break;
					}
				}
			}
		}
	});

	let (a, b) = tokio::join!(stdout_task, stderr_task);
	a.map_err(|it| it.to_string())?;
	b.map_err(|it| it.to_string())?;

	Ok(())
}
