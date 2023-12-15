use {
	crate::error::Error,
	serde::{
		de::Visitor,
		Deserialize,
		Deserializer,
		Serialize,
		Serializer,
	},
	std::{
		collections::HashMap,
		ffi::OsStr,
		fmt::{
			Display,
			Formatter,
		},
		path::{
			Component,
			PathBuf,
		},
	},
};

#[derive(Debug, Clone)]
pub struct ArtifactName {
	pub package: PathBuf,
	pub name: String,
	pub version: String,
}

impl ArtifactName {
	pub fn to_path(&self) -> PathBuf {
		self.package.join(&self.name).join(&self.version)
	}
}

impl Display for ArtifactName {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let components: Vec<&OsStr> = self
			.package
			.components()
			.map(|it| match it {
				Component::Normal(name) => Ok(name),
				_ => unreachable!("Invalid path component"),
			})
			.collect::<Result<_, std::fmt::Error>>()?;

		write!(
			f,
			"{}:{}:{}",
			components
				.join(OsStr::new("."))
				.to_str()
				.expect("Invalid os string"),
			self.name,
			self.version
		)
	}
}

impl<'de> Deserialize<'de> for ArtifactName {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		struct ArtifactNameVisitor;

		impl<'vis> Visitor<'vis> for ArtifactNameVisitor {
			type Value = ArtifactName;

			fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
				write!(
					formatter,
					"a string in artifact format `org.name.pkg:artifact:version`"
				)
			}

			fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
			where
				E: serde::de::Error,
			{
				let parts: Vec<&str> = v.split(':').collect();

				Ok(ArtifactName {
					package: parts[0].split('.').collect(),
					name: parts[1].into(),
					version: parts[2].into(),
				})
			}

			fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
			where
				E: serde::de::Error,
			{
				self.visit_str(&v)
			}
		}

		deserializer.deserialize_string(ArtifactNameVisitor)
	}
}

impl Serialize for ArtifactName {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		serializer.serialize_str(&format!("{self}"))
	}
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Os {
	Windows,
	Linux,
	Osx,
}

impl Os {
	pub fn target() -> Result<Self, Error> {
		if cfg!(target_os = "linux") {
			Ok(Self::Linux)
		} else if cfg!(target_os = "windows") {
			Ok(Self::Windows)
		} else if cfg!(target_os = "macos") {
			Ok(Self::Osx)
		} else {
			Err(Error::UnsupportedPlatform)
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Arch {
	X64,
	X86,
}

impl Arch {
	pub fn target() -> Self {
		if cfg!(any(target_arch = "x86_64", target_arch = "aarch64")) {
			Self::X64
		} else {
			Self::X86
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RuleAction {
	Allow,
	Disallow,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RuleCondition {
	Os {
		name: Option<Os>,
		arch: Option<Arch>,
		version: Option<String>,
	},
	Features(HashMap<String, bool>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
	pub action: RuleAction,
	#[serde(flatten)]
	pub condition: Option<RuleCondition>,
}

impl Rule {
	pub fn unpack_all(rules: &[Rule]) -> bool {
		rules.iter().all(Rule::unpack)
	}

	pub fn unpack(&self) -> bool {
		let mut allow = true;

		match &self.condition {
			Some(RuleCondition::Os { name, arch, .. }) => {
				if let Some(os_name) = &name {
					allow = match os_name {
						#[cfg(target_os = "linux")]
						Os::Linux => true,
						#[cfg(target_os = "windows")]
						Os::Windows => true,
						#[cfg(target_os = "macos")]
						Os::Osx => true,
						#[allow(unreachable_patterns)]
						_ => false,
					};
				}

				if let Some(os_arch) = &arch {
					allow = match os_arch {
						#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
						Arch::X64 => true,
						#[cfg(target_arch = "x86")]
						Arch::X86 => true,
						#[allow(unreachable_patterns)]
						_ => false,
					};
				}
			}
			Some(RuleCondition::Features(_)) => allow = false,
			_ => {}
		};

		match self.action {
			RuleAction::Allow => allow,
			RuleAction::Disallow => !allow,
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artifact {
	pub path: PathBuf,
	pub sha1: String,
	pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
	pub sha1: String,
	pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommonLibraryArtifacts {
	pub artifact: Artifact,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NativeLibraryArtifacts {
	pub artifact: Artifact,
	pub classifiers: HashMap<String, Artifact>,
}

impl NativeLibraryArtifacts {
	pub fn extract_artifact(&mut self, classifier_name: &str) -> Result<Artifact, Error> {
		self
			.classifiers
			.remove(classifier_name)
			.ok_or(Error::InvalidManifest("Missing native classifier".into()))
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommonLibrary {
	pub name: ArtifactName,
	pub downloads: CommonLibraryArtifacts,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NativeLibrary {
	pub name: ArtifactName,
	pub downloads: NativeLibraryArtifacts,
	pub natives: Natives,
	pub rules: Vec<Rule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Natives {
	#[serde(flatten)]
	pub inner: HashMap<Os, String>,
}

impl Natives {
	pub fn get_classifier_name(&self) -> Result<&str, Error> {
		Ok(
			self
				.inner
				.get(&Os::target()?)
				.ok_or(Error::InvalidManifest("Missing native classifier".into()))?
				.as_str(),
		)
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeminativeLibrary {
	pub name: ArtifactName,
	pub downloads: CommonLibraryArtifacts,
	pub rules: Vec<Rule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomLibrary {
	pub name: ArtifactName,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Library {
	Native(NativeLibrary),
	Seminative(SeminativeLibrary),
	Common(CommonLibrary),
	Custom(CustomLibrary),
}

impl Library {
	pub fn to_path(&self) -> PathBuf {
		match self {
			Library::Native(it) => it.name.to_path(),
			Library::Seminative(it) => it.name.to_path(),
			Library::Common(it) => it.name.to_path(),
			Library::Custom(it) => it.name.to_path(),
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ClientLogging {
	pub argument: String,
	#[serde(rename = "type")]
	pub log_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Logging {
	pub client: ClientLogging,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JavaVersion {
	pub component: String,
	pub major_version: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssetIndexResource {
	pub id: String,
	pub sha1: String,
	pub size: u64,
	pub total_size: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ConditionalArgument {
	Single(String),
	List(Vec<String>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Argument {
	Constant(String),
	Conditional {
		rules: Vec<Rule>,
		value: ConditionalArgument,
	},
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct PackageDownloads {
	pub client: Resource,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Args {
	pub game: Vec<Argument>,
	pub jvm: Vec<Argument>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModernArgs {
	pub arguments: Args,
}

impl From<String> for ModernArgs {
	fn from(value: String) -> Self {
		Self {
			arguments: Args {
				game: value
					.split_whitespace()
					.map(|it| Argument::Constant(it.to_owned()))
					.collect(),
				jvm: Vec::new(),
			},
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LegacyArgs {
	#[serde(rename = "minecraftArguments")]
	pub arguments: String,
}

impl From<LegacyArgs> for ModernArgs {
	fn from(value: LegacyArgs) -> Self {
		value.arguments.into()
	}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ArgsContainer {
	Modern(ModernArgs),
	Legacy(LegacyArgs),
}

impl From<ArgsContainer> for ModernArgs {
	fn from(value: ArgsContainer) -> Self {
		match value {
			ArgsContainer::Modern(it) => it,
			ArgsContainer::Legacy(it) => it.into(),
		}
	}
}

impl ArgsContainer {
	fn merge(self, with: ArgsContainer) -> Self {
		use ArgsContainer::*;

		match with {
			Modern(ModernArgs { arguments }) => {
				let Args {
					jvm: jvm_ext,
					game: game_ext,
				} = arguments;
				let mut modern = self.into_modern();
				let Args {
					ref mut jvm,
					ref mut game,
				} = modern.arguments;

				jvm.extend(jvm_ext);
				game.extend(game_ext);

				Modern(modern)
			}
			Legacy(args) => self.merge(Modern(args.into())),
		}
	}

	fn into_modern(self) -> ModernArgs {
		use ArgsContainer::*;

		match self {
			Modern(it) => it,
			Legacy(it) => it.into(),
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeManifest {
	pub id: String,
	pub assets: String,
	pub asset_index: AssetIndexResource,
	pub downloads: PackageDownloads,
	pub main_class: String,
	pub java_version: JavaVersion,
	#[serde(flatten)]
	pub arguments: ArgsContainer,
	pub libraries: Vec<Library>,
	pub logging: Logging,
	#[serde(rename = "type")]
	pub version_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InheritedManifest {
	pub id: String,
	pub inherits_from: String,
	pub main_class: String,
	#[serde(flatten)]
	pub arguments: ArgsContainer,
	pub libraries: Vec<Library>,
	#[serde(rename = "type")]
	pub version_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Manifest {
	Inherited(Box<InheritedManifest>),
	Native(Box<NativeManifest>),
}

impl From<Box<NativeManifest>> for Manifest {
	fn from(value: Box<NativeManifest>) -> Self {
		Manifest::Native(value)
	}
}

impl From<Box<InheritedManifest>> for Manifest {
	fn from(value: Box<InheritedManifest>) -> Self {
		Manifest::Inherited(value)
	}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AssetObject {
	pub hash: String,
	pub size: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AssetIndex {
	pub objects: HashMap<String, AssetObject>,
}
