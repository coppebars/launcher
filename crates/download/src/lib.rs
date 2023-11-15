use {
	futures::{
		stream::iter as fut_iter,
		StreamExt,
		TryStreamExt,
  },
	reqwest::Client,
	serde::Serialize,
	std::{
		path::PathBuf,
		sync::Arc,
  },
	thiserror::Error,
	tokio::{
		fs::File,
		io::AsyncWriteExt,
		sync::mpsc::Sender,
	},
	tokio_util::sync::CancellationToken,
	url::Url,
};

#[derive(Debug, Serialize, Clone)]
pub struct Item {
  pub url: Url,
  pub path: PathBuf,
  pub known_size: Option<u64>,
  pub known_sha: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Chunk {
  size: usize,
  total: Option<u64>,
  progress: usize,
}

#[derive(Debug, Error, Serialize, Clone)]
pub enum DownloadError {
  #[error("known: Unknown kind value")]
  UnknownKind,

  #[error("io: {0}")]
  Io(String),

  #[error("reqwest: {0}")]
  Reqwest(String),

  #[error("sync: {0}")]
  SendError(String),

	#[error("join: {0}")]
	JoinError(String),

  #[error("unexpected: {0}")]
  Unexpected(String),

  #[error("shutdown")]
  Shutdown,

  #[error("cancelled")]
  Cancelled,
}

macro_rules! from_err {
	($($t:ty => $id:ident),+) => {
		$(
			impl From<$t> for DownloadError {
				fn from(value: $t) -> Self {
					Self::$id(value.to_string())
				}
			}
		)+
	};
}

from_err! {
  std::io::Error => Io,
  reqwest::Error => Reqwest,
  tokio::sync::mpsc::error::SendError<DownloadEvent> => SendError,
	tokio::task::JoinError => JoinError
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum DownloadEvent {
  Start { item: Item },
  Chunk { path: String, chunk: Chunk },
  Error { item: Item, error: DownloadError },
  Finish { item: Item },
}

pub async fn download(
  client: &Client,
  item: Item,
  sender: &Sender<DownloadEvent>,
  token: &CancellationToken,
) -> Result<(), DownloadError> {
  if token.is_cancelled() {
    return Err(DownloadError::Cancelled);
  }

  sender
    .send(DownloadEvent::Start { item: item.clone() })
    .await?;

  let response = client.get(item.url.to_owned()).send().await?;

  let content_length = response.content_length();
  let mut stream = response.bytes_stream().map_err(DownloadError::from);
  let mut target_file = File::create(&item.path).await?;

  let mut progress: usize = 0;

  let path_key = item
    .path
    .to_str()
    .ok_or(DownloadError::Unexpected(
      "Failed to convert path to string".into(),
    ))?
    .to_owned();

  while let Some(bytes) = stream.next().await {
    if token.is_cancelled() {
      return Err(DownloadError::Cancelled);
    }

    let bytes = bytes?;
    target_file.write_all(&bytes).await?;

    progress += bytes.len();

    sender
      .send(DownloadEvent::Chunk {
        path: path_key.clone(),
        chunk: Chunk {
          progress,
          size: bytes.len(),
          total: content_length,
        },
      })
      .await?;
  }

  sender.send(DownloadEvent::Finish { item }).await?;

  Ok(())
}

pub async fn download_all(
  client: &Client,
  items: Vec<Item>,
  sender: Arc<Sender<DownloadEvent>>,
  token: Arc<CancellationToken>,
  workers: usize,
) -> Result<(), DownloadError> {
  let mut futures = fut_iter(items.into_iter().map(|it| {
    let client = client.clone();
    let sender = sender.clone();
    let token = token.clone();

    tokio::spawn(async move {
			match download(&client, it.clone(), &sender, &token).await {
				Ok(result) => Ok(result),
				Err(err) => {
					sender.send(DownloadEvent::Error { item: it.clone(), error: err.clone() }).await?;
					Err(err)
				}
			}
		})
  }))
  .buffer_unordered(workers);

	while let Some(result) = futures.next().await {
		result??
	}

  Ok(())
}
