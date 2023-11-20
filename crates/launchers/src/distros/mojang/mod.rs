use {
	crate::{
		launch::{
			utils,
			ProcessLauncher,
		},
		specs::manifest::{
			Manifest,
			ModernArgs,
		},
	},
	once_cell::sync::Lazy,
	serde::{
		Serialize,
		Serializer,
	},
	std::{
		collections::HashSet,
		io::Read,
		path::{
			Path,
			PathBuf,
		},
	},
	thiserror::Error,
};

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
}

static DEFAULT_FEATURES: Lazy<HashSet<&str>> =
	Lazy::new(|| HashSet::from(["has_custom_resolution"]));

impl Mojang {
	#[cfg(feature = "serde_json")]
	pub fn try_from_json(value: &str) -> Result<Self, serde_json::Error> {
		Ok(Self(serde_json::from_str(value)?))
	}

	#[cfg(all(not(feature = "async"), feature = "serde_json"))]
	pub fn try_from_file(path: &Path) -> Result<Self, TryFromError> {
		let mut file = std::fs::File::open(path)?;
		let mut contents = String::with_capacity(8192);

		file.read_to_string(&mut contents)?;

		Ok(Self::try_from_json(&contents)?)
	}

	#[cfg(all(feature = "async", feature = "serde_json"))]
	pub async fn try_from_file(path: &Path) -> Result<Self, TryFromError> {
		use tokio::io::AsyncReadExt;

		let mut file = tokio::fs::File::open(path).await?;
		let mut contents = String::with_capacity(8192);

		file.read_to_string(&mut contents).await?;

		Ok(Self::try_from_json(&contents)?)
	}

	pub fn try_into_process(self) -> Result<ProcessLauncher, Error> {
		let manifest = match self.0 {
			Manifest::Root(it) => it,
			Manifest::Inherited(_) => return Err(Error::Inherited),
		};

		let mut classpath = utils::libraries_to_classpath(manifest.libraries);

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
}
