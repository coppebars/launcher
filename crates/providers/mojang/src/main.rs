use {
	download::{
		download_all,
		DownloadEvent,
  },
	reqwest::Client,
	std::{
		path::Path,
		sync::Arc,
		time::Duration,
  },
	tokio::sync::mpsc::channel,
	tokio_util::sync::CancellationToken,
};

mod api;
mod install;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let items = install::get_items("1.16.4").await?;

  let root = Path::new("./minecraft");

  let items: Vec<_> = items.into_iter().map(|it| it.place(root)).collect();

  let client = Client::builder().connect_timeout(Duration::from_secs(30)).build()?;

  let (tx, mut rx) = channel::<DownloadEvent>(1024);
  let token = Arc::new(CancellationToken::new());

  let task_token = token.clone();
  let task =
    tokio::spawn(async move { download_all(&client, items, Arc::new(tx), task_token, 4).await });

  while let Some(msg) = rx.recv().await {
    if let DownloadEvent::Finish {
      total, progress, ..
    } = msg
    {
      println!("{progress} / {total}")
    };
  }

  task.await??;

  Ok(())
}
