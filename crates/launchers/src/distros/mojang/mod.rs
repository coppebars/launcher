use tokio::io::AsyncWriteExt;
use {
	crate::{
		distros::{
			mojang::api::get_jre_components,
			PrepareAction,
			PrepareRemoteFileAction,
			PrepareWriteAction,
		},
		launch::{
			utils,
			ProcessLauncher,
		},
		specs::{
			jre::{
				ComponentType,
				Entry,
				Manifest as JreManifest,
				Target,
			},
			manifest::{
				AssetIndex,
				Library,
				Manifest,
				ModernArgs,
				Os,
				RootManifest,
				Rule,
			},
		},
	},
	once_cell::sync::Lazy,
	serde::{
		Serialize,
		Serializer,
	},
	std::{
		collections::HashSet,
		fmt::{
			Display,
			Formatter,
		},
		path::{
			Path,
			PathBuf,
		},
	},
	thiserror::Error,
	url::Url,
	tokio::fs,
};

mod api;

#[derive(Debug, Clone)]
pub struct Mojang(Manifest);

#[derive(Debug, Error)]
pub enum TryFromError {
	#[error(transparent)]
	Io(#[from] std::io::Error),

	#[error(transparent)]
	Json(#[from] serde_json::Error),
}

impl Serialize for TryFromError {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		serializer.serialize_str(self.to_string().as_str())
	}
}

#[derive(Debug, Error)]
pub enum Error {
	#[error("Target manifest is inheriting. It does not contain enough data to to launch from it")]
	Inherited,

	#[error("Invalid utf-8 in path")]
	InvalidUtf8Path,

	#[error("Manifest with id: {0}, not found")]
	ManifestNotFound(String),

	#[error(transparent)]
	Reqwest(#[from] reqwest::Error),

	#[error(transparent)]
	Json(#[from] serde_json::Error),

	#[error(transparent)]
	UrlParse(#[from] url::ParseError),

	#[error("Manifest malformed: {0}")]
	Malformed(String),

	#[error(transparent)]
	Io(#[from] std::io::Error)
}

static DEFAULT_FEATURES: Lazy<HashSet<&str>> =
	Lazy::new(|| HashSet::from(["has_custom_resolution"]));

impl Display for PrepareAction {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		use PrepareAction::*;

		match self {
			RemoteFile(it) => {
				write!(f, "RemoteFile: {} => {}", it.url, it.path.to_str().unwrap())
			}
			Write(it) => {
				write!(f, "Write: {}", it.path.to_str().unwrap())
			}
		}
	}
}

impl Mojang {
	#[cfg(feature = "serde_json")]
	pub fn try_from_json(value: &str) -> Result<Self, serde_json::Error> {
		Ok(Self(serde_json::from_str(value)?))
	}

	#[cfg(feature = "serde_json")]
	pub async fn try_from_file(path: &Path) -> Result<Self, TryFromError> {
		use tokio::io::AsyncReadExt;

		let mut file = tokio::fs::File::open(path).await?;
		let mut contents = String::with_capacity(8192);

		file.read_to_string(&mut contents).await?;

		Ok(Self::try_from_json(&contents)?)
	}

	pub async fn try_from_canonical_tree(root: &Path, id: &str) -> Result<Self, TryFromError> {
		Self::try_from_file(&root.join("versions").join(id).join(format!("{}.json", id))).await
	}

	pub fn try_into_process(self) -> Result<ProcessLauncher, Error> {
		let manifest = match self.0 {
			Manifest::Root(it) => it,
			Manifest::Inherited(_) => return Err(Error::Inherited),
		};

		let classpath = utils::libraries_to_classpath(manifest.libraries);

		let ModernArgs { arguments }: ModernArgs = manifest.arguments.into();

		let mut process = ProcessLauncher {
			bin: PathBuf::from("./jre")
				.join(manifest.java_version.component)
				.join("bin/java"),
			main_class: manifest.main_class,
			..Default::default()
		};

		utils::process_args(arguments.jvm, &mut process.jvm_args, &DEFAULT_FEATURES);
		utils::process_args(arguments.game, &mut process.game_args, &DEFAULT_FEATURES);

		process
			.jvm_args
			.retain(|it| !it.as_str().starts_with("-Djava.library.path"));

		let libraries_base = PathBuf::from("libraries");

		let mut classpath: Vec<_> = classpath
			.into_iter()
			.map(|it| libraries_base.join(it))
			.collect();

		let version_base = PathBuf::from("versions").join(&manifest.id);

		classpath.push(version_base.join(format!("{}.jar", &manifest.id)));

		process.vars.insert(
			"classpath".into(),
			utils::join_classpath(
				&classpath
					.iter()
					.map(|it| {
						it.to_str()
							.ok_or(Error::InvalidUtf8Path)
							.map(|it| it.to_owned())
					})
					.collect::<Result<Vec<_>, _>>()?,
			),
		);

		process.vars.insert("version_name".into(), manifest.id);
		process
			.vars
			.insert("version_type".into(), manifest.version_type);
		process
			.vars
			.insert("assets_index_name".into(), manifest.assets);
		process.vars.insert("assets_root".into(), "assets".into());
		process.vars.insert(
			"natives_directory".into(),
			version_base
				.join("natives")
				.to_str()
				.ok_or(Error::InvalidUtf8Path)?
				.to_owned(),
		);

		Ok(process)
	}

	pub async fn save_manifest(root: &Path, id: &str) -> Result<Box<RootManifest>, Error> {
		let manifest = Self::get_manifest(id).await?;
		let version_dir = root.join("versions").join(id);
		fs::create_dir_all(&version_dir).await?;
		let mut file = fs::File::create(version_dir.join(format!("{}.json", id))).await?;

		file.write_all(serde_json::to_string(&manifest)?.as_bytes()).await?;

		Ok(manifest)
	}

	pub async fn get_manifest(id: &str) -> Result<Box<RootManifest>, Error> {
		let versions = api::get_versions().await?.versions;
		let version = versions
			.into_iter()
			.find(|it| it.id == id)
			.ok_or(Error::ManifestNotFound(id.to_owned()))?;

		Ok(reqwest::get(version.url).await?.json().await?)
	}

	#[cfg(feature = "serde_json")]
	pub async fn prepare(self) -> Result<Vec<PrepareAction>, Error> {
		let mut actions = Vec::with_capacity(4096);

		let versions_dir = PathBuf::from("versions");
		let assets_dir = PathBuf::from("assets");
		let libraries_dir = PathBuf::from("libraries");
		let jre_dir = PathBuf::from("jre");

		let manifest = match self.0 {
			Manifest::Root(it) => it,
			Manifest::Inherited(it) => {
				let id = it.id.clone();
				let root = it.into_root(Self::get_manifest(&id).await?);

				actions.push(PrepareAction::Write(PrepareWriteAction {
					path: versions_dir.join(id),
					content: serde_json::to_string(&root)?.into_bytes(),
				}));

				root
			}
		};

		let jre_component_dir = jre_dir.join(&manifest.java_version.component);

		macro_rules! push_action {
			($url:expr, $path:expr) => {
				actions.push(PrepareAction::RemoteFile(PrepareRemoteFileAction {
					url: $url,
					path: $path,
					known_size: None,
					known_sha: None,
				}))
			};
			($url:expr, $path:expr, $size:expr) => {
				actions.push(PrepareAction::RemoteFile(PrepareRemoteFileAction {
					url: $url,
					path: $path,
					known_size: Some($size),
					known_sha: None,
				}))
			};
			($url:expr, $path:expr, $size:expr, $sha:expr) => {
				actions.push(PrepareAction::RemoteFile(PrepareRemoteFileAction {
					url: $url,
					path: $path,
					known_size: Some($size),
					known_sha: Some($sha),
				}))
			};
		}

		let version_dir = versions_dir.join(&manifest.id);

		push_action!(
			manifest.downloads.client.url,
			version_dir.join(format!("{}.jar", &manifest.id))
		);

		for lib in manifest.libraries {
			use Library::*;

			match lib {
				Custom { name, url } => {
					let path = utils::libname_to_path(&name).unwrap();

					let url = url.join(path.to_str().unwrap())?;

					push_action!(url, libraries_dir.join(path))
				}
				Native {
					rules,
					mut downloads,
					natives,
					..
				} => {
					push_action!(
						downloads.artifact.url,
						libraries_dir.join(downloads.artifact.path)
					);

					if !rules.iter().all(Rule::unwrap) {
						continue;
					}

					let classifier = if cfg!(target_os = "windows") {
						natives.get(&Os::Windows)
					} else if cfg!(target_os = "linux") {
						natives.get(&Os::Linux)
					} else if cfg!(target_os = "macos") {
						natives.get(&Os::Osx)
					} else {
						panic!("Unsupported os")
					};

					let classifier =
						classifier.ok_or(Error::Malformed("Inappropriate native classifier".into()))?;

					let artifact = downloads
						.classifiers
						.remove(classifier)
						.ok_or(Error::Malformed("Inappropriate native classifier".into()))?;

					push_action!(
						artifact.url,
						libraries_dir.join(artifact.path),
						artifact.size,
						artifact.sha1
					);
				}
				Seminative {
					rules, downloads, ..
				} => {
					if !rules.iter().all(Rule::unwrap) {
						continue;
					}

					push_action!(
						downloads.artifact.url,
						libraries_dir.join(downloads.artifact.path),
						downloads.artifact.size,
						downloads.artifact.sha1
					);
				}
				Default { downloads, .. } => {
					push_action!(
						downloads.artifact.url,
						libraries_dir.join(downloads.artifact.path),
						downloads.artifact.size,
						downloads.artifact.sha1
					);
				}
			}
		}

		let asset_index: AssetIndex = reqwest::get(manifest.asset_index.url).await?.json().await?;

		for it in asset_index.objects.into_values() {
			let path = format!("{}/{}", &it.hash[..2], it.hash);

			push_action!(
				Url::parse(&format!(
					"https://resources.download.minecraft.net/{}",
					path
				))?,
				assets_dir.join(path),
				it.size,
				it.hash
			);
		}

		let jre_manifest = get_jre_components().await?;
		let jre_target = jre_manifest
			.get(
				#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
				&Target::Linux,
				#[cfg(all(target_os = "linux", target_arch = "x86"))]
				&Target::LinuxI386,
				#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
				&Target::WindowsX64,
				#[cfg(all(target_os = "windows", target_arch = "x86"))]
				&Target::WindowsX86,
				#[cfg(all(target_os = "windows", target_arch = "aarch64"))]
				&Target::WindowsArm64,
				#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
				&Target::Macos,
				#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
				&Target::MacosArm64,
			)
			.expect("Target is not supported");

		let jre_component = jre_target
			.get(
				&ComponentType::from_str(&manifest.java_version.component)
					.ok_or(Error::Malformed("Unknown jre component".into()))?,
			)
			.ok_or(Error::Malformed("No such jre component".into()))?
			.get(0)
			.ok_or(Error::Malformed(
				"Jre is not supported for current platform".into(),
			))?;

		let jre_manifest: JreManifest = reqwest::get(jre_component.manifest.url.clone())
			.await?
			.json()
			.await?;

		for (path, entry) in jre_manifest.files {
			if let Entry::File(it) = entry {
				let file = it.downloads.raw;

				push_action!(file.url, jre_component_dir.join(path), file.size, file.sha1)
			}
		}

		Ok(actions)
	}
}
