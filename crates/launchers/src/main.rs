use {
	download::DownloadEvent,
	launchers::distros::mojang::Mojang,
	std::{
		path::Path,
		sync::Arc,
	},
	tokio_util::sync::CancellationToken,
};

#[tokio::main]
async fn main() {
	let distro = Mojang::try_from_file(Path::new("./minecraft/versions/1.16.4/1.16.4.json"))
		.await
		.unwrap();

	let mut process = distro.try_into_process().unwrap();

	process.cwd = "./minecraft".into();

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
