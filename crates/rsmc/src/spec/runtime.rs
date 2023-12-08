use {
	serde::Deserialize,
	std::{
		collections::HashMap,
		path::PathBuf,
	},
	url::Url,
};

#[derive(Debug, Deserialize, PartialEq, Eq, Hash)]
pub enum Target {
	#[serde(rename = "gamecore")]
	GameCore,
	#[serde(rename = "linux")]
	Linux,
	#[serde(rename = "linux-i386")]
	LinuxI386,
	#[serde(rename = "mac-os")]
	Macos,
	#[serde(rename = "mac-os-arm64")]
	MacosArm64,
	#[serde(rename = "windows-arm64")]
	WindowsArm64,
	#[serde(rename = "windows-x64")]
	WindowsX64,
	#[serde(rename = "windows-x86")]
	WindowsX86,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum ComponentType {
	JavaRuntimeAlpha,
	JavaRuntimeBeta,
	JavaRuntimeGamma,
	JavaRuntimeGammaSnapshot,
	JreLegacy,
	MinecraftJavaExe,
}

impl ComponentType {
	pub fn from_str(value: &str) -> ComponentType {
		match value {
			"java-runtime-alpha" => Self::JavaRuntimeAlpha,
			"java-runtime-beta" => Self::JavaRuntimeBeta,
			"java-runtime-gamma" => Self::JavaRuntimeGamma,
			"java-runtime-gamma-snapshot" => Self::JavaRuntimeGammaSnapshot,
			"jre-legacy" => Self::JreLegacy,
			"minecraft-java-exe" => Self::MinecraftJavaExe,
			_ => unreachable!(),
		}
	}
}

#[derive(Debug, Deserialize)]
pub struct ManifestResource {
	pub sha1: String,
	pub size: u32,
	pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Availability {
	pub group: u32,
	pub progress: u32,
}

#[derive(Debug, Deserialize)]
pub struct Version {
	pub name: String,
	pub released: String,
}

#[derive(Debug, Deserialize)]
pub struct Component {
	pub availability: Availability,
	pub manifest: ManifestResource,
	pub version: Version,
}

#[derive(Debug, Deserialize)]
pub struct JreFile {
	pub sha1: String,
	pub size: u64,
	pub url: Url,
}

#[derive(Debug, Deserialize)]
pub struct Downloads {
	pub lzma: Option<JreFile>,
	pub raw: JreFile,
}

#[derive(Debug, Deserialize)]
pub struct FileEntry {
	pub executable: bool,
	pub downloads: Downloads,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum Entry {
	Link,
	Directory,
	File(Box<FileEntry>),
}

pub type Files = HashMap<PathBuf, Entry>;

#[derive(Debug, Deserialize)]
pub struct Manifest {
	pub files: Files,
}
