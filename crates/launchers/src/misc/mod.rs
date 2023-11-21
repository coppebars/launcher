use {
	crate::distros::PrepareAction,
	reqwest::Client,
	std::{
		path::Path,
		sync::Arc,
		time::Duration,
	},
	tokio::sync::mpsc::Sender,
	tokio_util::sync::CancellationToken,
};
#[cfg(feature = "download")]
use download::{
	download_all,
	DownloadError,
	DownloadEvent,
	Item,
};

#[cfg(feature = "download")]
pub async fn place_to_canonical_tree(
	root: &Path,
	actions: Vec<PrepareAction>,
	sender: Arc<Sender<DownloadEvent>>,
	token: Arc<CancellationToken>,
) -> Result<(), DownloadError> {
	let items: Vec<_> = actions
		.iter()
		.filter_map(|it| match it {
			PrepareAction::RemoteFile(it) => Some(Item {
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
