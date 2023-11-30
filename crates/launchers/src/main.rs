use {
	launchers::distros::mojang::Mojang,
	std::path::Path,
};

#[tokio::main]
async fn main() {
	let distro = Mojang::try_from_file(Path::new("./minecraft/versions/1.20.1/1.20.1.json"))
		.await
		.unwrap();

	let mut process = distro.try_into_process().unwrap();

	process.cwd = "./minecraft".into();
	//process.jvm_args.push("-Dlog4j.configurationFile=${path}".into());
	process
		.vars
		.insert("auth_player_name".into(), "LIMPIX31".into());
	process.vars.insert(
		"auth_uuid".into(),
		"00000000-0000-0000-0000-000000000000".into(),
	);
	process.vars.insert(
		"minecraft_account_host".into(),
		"https://nodium.ru:9000/account".into(),
	);
	process.vars.insert(
		"minecraft_auth_host".into(),
		"https://nodium.ru:9000/auth".into(),
	);
	process.vars.insert(
		"minecraft_session_host".into(),
		"https://nodium.ru:9000/session".into(),
	);
	process.vars.insert(
		"minecraft_services_host".into(),
		"https://nodium.ru:9000/services".into(),
	);
	process.vars.insert("user_type".into(), "msa".into());
	process
		.vars
		.insert("auth_access_token".into(), "test".into());
	//process.vars.insert("path".into(),
	// "/home/limpix/workspaces/launcher/log4j.xml".into());

	process.launch_blocking();

	// let actions = distro.prepare().await.unwrap();
	//
	// let (tx, mut rx) = tokio::sync::mpsc::channel(1024);
	// let token = Arc::new(CancellationToken::new());
	//
	// let task_token = token.clone();
	//
	// let task = tokio::spawn(launchers::misc::place_to_canonical_tree(
	// 	Path::new("./minecraft"),
	// 	actions,
	// 	Arc::new(tx),
	// 	task_token,
	// ));
	//
	// while let Some(msg) = rx.recv().await {
	// 	if let DownloadEvent::Finish {
	// 		total, progress, ..
	// 	} = msg
	// 	{
	// 		println!("{progress} / {total}")
	// 	};
	// }
	//
	// task.await.unwrap().unwrap();
}
