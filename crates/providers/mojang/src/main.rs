use std::fs;
use {
  crate::install::{
    Item,
    Kind,
  },
  futures::{
    StreamExt,
    TryStreamExt,
  },
  std::path::PathBuf,
  tokio::{
		self,
    fs::File,
    io::AsyncWriteExt,
  },
};

mod api;
mod install;

async fn download(item: Item) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
	println!("{:?}", &item.url.to_string());
	let response = reqwest::get(item.url).await?;
	let content_length = response.content_length();
	let mut stream = response.bytes_stream().map_err(|_| "");
  let path = match item.kind {
    Kind::Lib => PathBuf::from("./minecraft/libraries").join(&item.path),
    Kind::Native => PathBuf::from("./minecraft/versions").join(&item.path),
    Kind::Asset => PathBuf::from("./minecraft/assets").join(&item.path),
    Kind::Version => PathBuf::from("./minecraft/versions").join(&item.path),
  };
	fs::create_dir_all(path.parent().unwrap())?;
  let mut target_file = File::create(path).await?;

  let mut collected: usize = 0;

  while let Some(bytes) = stream.next().await {
    let bytes = bytes?;
    target_file.write_all(&bytes).await?;

		println!("{:?} :: {:?} / {:?}", &item.path, &collected, &content_length);

    collected += bytes.len();
  }

  Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let items = install::get_items("1.20.1").await?;

  let futures = items.into_iter().map(|it| {
    tokio::spawn(async move { download(it).await })
  });
  let futures = futures::stream::iter(futures)
    .buffer_unordered(8)
    .collect::<Vec<_>>();

  futures.await;

  Ok(())
}
