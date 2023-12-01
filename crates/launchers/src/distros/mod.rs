use {
	crate::specs::manifest::Manifest,
	std::{
		io::ErrorKind,
		path::{
			Path,
			PathBuf,
		},
	},
	tokio::{
		fs,
		io::{
			AsyncReadExt,
			AsyncWriteExt,
		},
	},
};

fn join_version_path(root: &Path, id: &str) -> PathBuf {
	root.join("versions").join(id).join(format!("{}.json", id))
}

#[cfg(feature = "mojang")]
pub mod mojang;

pub trait VersionId {
	fn version_id(&self) -> String;
}

pub trait Distro
where
	Self: Sized + Send + Sync,
{
	type Error: From<std::io::Error> + From<serde_json::Error>;
	type VersionOptions: VersionId;

	fn from_manifest(value: Manifest) -> Self;

	fn try_from_json(value: &str) -> Result<Self, Self::Error>;

	async fn fetch_manifest(options: &Self::VersionOptions) -> Result<Manifest, Self::Error>;

	async fn try_from_file(path: &Path) -> Result<Self, Self::Error> {
		let mut file = fs::File::open(path).await?;
		let mut contents = String::with_capacity(8192);

		file.read_to_string(&mut contents).await?;

		Self::try_from_json(&contents)
	}

	async fn try_from_root(root: &Path, id: &str) -> Result<Self, Self::Error> {
		Self::try_from_file(&join_version_path(root, id)).await
	}

	async fn install_manifest(
		root: &Path,
		options: &Self::VersionOptions,
	) -> Result<Manifest, Self::Error> {
		let version_id = options.version_id();

		let manifest = Self::fetch_manifest(options).await?;
		let version_dir = root.join("versions").join(&version_id);
		fs::create_dir_all(&version_dir).await?;
		let mut file = fs::File::create(version_dir.join(format!("{}.json", &version_id))).await?;

		file
			.write_all(serde_json::to_string(&manifest)?.as_bytes())
			.await?;

		Ok(manifest)
	}

	async fn try_from_root_else_install(
		root: &Path,
		options: &Self::VersionOptions,
	) -> Result<Self, Self::Error> {
		let version_id = options.version_id();

		let path = join_version_path(root, &version_id);

		Ok(match fs::File::open(path).await {
			Ok(mut file) => {
				let mut contents = String::with_capacity(8192);

				file.read_to_string(&mut contents).await?;

				Ok(Self::try_from_json(&contents)?)
			}
			Err(err) => match err.kind() {
				ErrorKind::NotFound => {
					let manifest = Self::install_manifest(root, options).await?;

					Ok(Self::from_manifest(manifest))
				}
				_ => Err(err),
			},
		}?)
	}
}
