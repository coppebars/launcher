use {
	launchers::{
		distros::mojang::Mojang,
		misc::DownloadEvent,
	},
	std::{
		path::Path,
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
pub async fn mojang_prepare(window: Window, id: &str, path: &Path) -> Result<(), String> {
	let distro = Mojang::try_from_canonical_tree(path, id)
		.await
		.map_err(|it| it.to_string())?;

	let actions = distro.prepare().await.map_err(|it| it.to_string())?;

	let (tx, mut rx) = mpsc::channel(1024);
	let token = Arc::new(CancellationToken::new());

	let task_token = token.clone();

	let task = tokio::spawn(launchers::misc::place_to_canonical_tree(
		Path::new("./minecraft"),
		actions,
		Arc::new(tx),
		task_token,
	));

	let now = SystemTime::now();
	let mut next = SystemTime::now().elapsed().unwrap();

	while let Some(msg) = rx.recv().await {
		let elapsed = now.elapsed().unwrap();
		match msg {
			DownloadEvent::Chunk { .. } => {
				if elapsed > next {
					next = elapsed + Duration::from_millis(1000);
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
