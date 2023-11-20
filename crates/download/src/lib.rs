use {
	futures::{
		stream::iter as fut_iter,
		StreamExt,
		TryStreamExt,
	},
	reqwest::Client,
	serde::{
		Serialize,
		Serializer,
	},
	std::{
		io::ErrorKind,
		path::PathBuf,
		sync::{
			atomic::{
				AtomicUsize,
				Ordering,
			},
			Arc,
		},
	},
	thiserror::Error,
	tokio::{
		fs::{
			self,
			File,
		},
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

#[derive(Debug, Error)]
pub enum DownloadError {
	#[error(transparent)]
	Io(#[from] std::io::Error),

	#[error(transparent)]
	Reqwest(#[from] reqwest::Error),

	#[error("Invalid unicode in path: {0}")]
	InvalidPathUnicode(PathBuf),

	#[error(transparent)]
	Integrity(#[from] integrity::IntegrityCheckError),

	#[error(transparent)]
	Send(#[from] tokio::sync::mpsc::error::SendError<DownloadEvent>),

	#[error(transparent)]
	Join(#[from] tokio::task::JoinError),

	#[error("Download cancelled")]
	Cancelled,
}

impl Serialize for DownloadError {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		serializer.serialize_str(self.to_string().as_str())
	}
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum DownloadEvent {
	Start {
		item: Item,
	},
	Chunk {
		path: String,
		size: usize,
		total: Option<u64>,
		progress: usize,
	},
	Finish {
		item: Item,
		total: usize,
		progress: usize,
	},
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

	if let Some(sha) = item.known_sha {
		match File::open(&item.path).await {
			Ok(mut file) => {
				if integrity::check(&mut file, &sha).await? {
					return Ok(());
				}
			}
			Err(err) => match err.kind() {
				ErrorKind::NotFound => {}
				_ => Err(err)?,
			},
		};
	}

	let response = client.get(item.url.to_owned()).send().await?;

	let content_length = response.content_length().or(item.known_size);
	let mut stream = response.bytes_stream().map_err(DownloadError::from);
	fs::create_dir_all(&item.path.parent().unwrap()).await?;
	let mut target_file = File::create(&item.path).await?;

	let mut progress: usize = 0;

	let path_key = item
		.path
		.to_str()
		.ok_or(DownloadError::InvalidPathUnicode(item.path.clone()))?
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
				progress,
				size: bytes.len(),
				total: content_length,
			})
			.await?;
	}

	Ok(())
}

pub async fn download_all(
	client: &Client,
	items: Vec<Item>,
	sender: Arc<Sender<DownloadEvent>>,
	token: Arc<CancellationToken>,
	workers: usize,
) -> Result<(), DownloadError> {
	let len = items.len();
	let counter = Arc::new(AtomicUsize::new(0));

	let mut futures = fut_iter(items.into_iter().map(|it| {
		let counter = counter.clone();
		let client = client.clone();
		let sender = sender.clone();
		let token = token.clone();

		tokio::spawn(async move {
			match download(&client, it.clone(), &sender, &token).await {
				Ok(result) => {
					counter.fetch_add(1, Ordering::Relaxed);

					sender
						.send(DownloadEvent::Finish {
							item: it.clone(),
							total: len,
							progress: counter.load(Ordering::Relaxed),
						})
						.await?;

					Ok(result)
				}
				Err(err) => Err(err),
			}
		})
	}))
	.buffer_unordered(workers);

	while let Some(result) = futures.next().await {
		result??
	}

	Ok(())
}
