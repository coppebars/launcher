use {
	download::{
		download_all,
		DownloadEvent,
		Item,
  },
	reqwest::Client,
	std::sync::Arc,
	tokio::sync::mpsc::channel,
	tokio_util::sync::CancellationToken,
	url::Url,
};

#[tokio::main]
async fn main() {
  let items = vec![
		Item {
			known_size: None,
			// known_sha: None,
			known_sha: Some("265ca2072f7c3a9e0dae8c4abe223431089d9980".into()),
			url: Url::parse("https://piston-data.mojang.com/v1/objects/265ca2072f7c3a9e0dae8c4abe223431089d9980/client.jar").unwrap(),
			path: "./minecraft/client.jar".into(),
		},
		Item {
			known_size: None,
			// known_sha: None,
			known_sha: Some("9c2b37701bf77ae22df4c32fd6dd1614049ce994".into()),
			url: Url::parse("https://piston-data.mojang.com/v1/objects/9c2b37701bf77ae22df4c32fd6dd1614049ce994/server.jar").unwrap(),
			path: "./minecraft/server.jar".into(),
		}
	];

  let client = Client::new();

  let (tx, mut rx) = channel::<DownloadEvent>(1024);
  let token = Arc::new(CancellationToken::new());

	let task_token = token.clone();
	let task = tokio::spawn(async move {
		download_all(&client, items, Arc::new(tx), task_token, 2).await
	});

	// tokio::spawn(async move {
	// 	tokio::time::sleep(Duration::from_secs(3)).await;
	// 	token.cancel();
	// });

	while let Some(msg) = rx.recv().await {
		if matches!(msg, DownloadEvent::Finish { .. }) {
			println!("{:#?}", msg)
		}
	}

	task.await.unwrap().unwrap();
}
