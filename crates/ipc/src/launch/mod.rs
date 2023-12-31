use {
	rsmc::launcher::Launcher,
	std::{
		collections::HashMap,
		path::PathBuf,
	},
	tauri::{
		AppHandle,
		Manager,
		Window,
	},
	tracing::{
		error,
		info,
		trace,
	},
};

#[tauri::command]
pub async fn launch(
	handle: AppHandle,
	window: Window,
	root: PathBuf,
	id: String,
	vars: HashMap<String, String>,
) -> Result<(), String> {
	trace!("Constructing launcher");
	let mut launcher = Launcher::try_from_root(&root, &id).await.map_err(|err| {
		error!(?err, "Failed to construct launcher from manifest");

		err.to_string()
	})?;

	// trace!("Getting authlib injector");
	// let authlib_injector_path = handle
	// 	.path()
	// 	.resolve("athl.jar", BaseDirectory::Resource)
	// 	.expect("Failed to resolve resource");
	//
	// debug!("Java Agent: {:?}", authlib_injector_path);
	//
	// let authlib_injector_path = authlib_injector_path.to_str().ok_or("Invalid
	// path")?;
	//
	// launcher
	// 	.extra_jvm_args
	// 	.extend([
	// 		format!("-javaagent:{authlib_injector_path}=https://nodium.ru:32717"),
	// 		"-Dauthlibinjector.disableHttpd".into(),
	// 		"-Dauthlibinjector.noShowServerName".into(),
	// 		"-Dauthlibinjector.usernameCheck=enabled".into()
	// 	]);

	trace!(?vars);
	launcher.vars = vars;

	let (stdo, stde, mut rx) = launcher.launch().await.map_err(|it| it.to_string())?;
	info!("Version {id} launched");

	while let Some(msg) = rx.recv().await {
		trace!("{id}: {}", msg.trim_end());
		window.emit("log", msg).map_err(|it| it.to_string())?;
	}

	let (a, b) = tokio::join!(stdo, stde);
	a.map_err(|it| it.to_string())?;
	b.map_err(|it| it.to_string())?;

	Ok(())
}
