use {
	crate::install::action::Action,
	download::{
		download_all,
		DownloadError,
		Item,
	},
	reqwest::Client,
	std::{
		path::PathBuf,
		sync::Arc,
		time::Duration,
	},
	tokio::sync::mpsc::Sender,
	tokio_util::sync::CancellationToken,
};

pub use download::DownloadEvent;

pub trait ActionExecutorExt {
	type Error;

	async fn execute(
		self,
		root: PathBuf,
		sender: Arc<Sender<DownloadEvent>>,
		token: Arc<CancellationToken>,
	) -> Result<(), Self::Error>;
}

impl ActionExecutorExt for Vec<Action> {
	type Error = DownloadError;

	async fn execute(
		self,
		root: PathBuf,
		sender: Arc<Sender<DownloadEvent>>,
		token: Arc<CancellationToken>,
	) -> Result<(), Self::Error> {
		let items: Vec<_> = self
			.iter()
			.filter_map(|it| match it {
				Action::Download(it) => Some(Item {
					url: it.url.clone(),
					path: root.join(&it.path),
					known_size: it.known_size,
					known_sha: it.known_sha.clone(),
				}),
				_ => None,
			})
			.collect();

		let client = Client::builder()
			.connect_timeout(Duration::from_secs(30))
			.build()?;

		download_all(&client, items, sender, token, 4).await
	}
}
