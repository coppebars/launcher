use {
	serde::Deserialize,
	std::{
		collections::HashMap,
		path::PathBuf,
	},
	url::Url,
};

#[derive(Debug, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Os {
	Linux,
	Windows,
	Osx,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Arch {
	X64,
	X86,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ConditionalArgument {
	Single(String),
	List(Vec<String>),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Argument {
	Constant(String),
	Conditional {
		rules: Vec<Rule>,
		value: ConditionalArgument,
	},
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum RuleAction {
	#[default]
	Allow,
	Disallow,
}

#[derive(Debug, Deserialize)]
pub struct Condition {
	pub os: Option<RuleOsCondition>,
	pub features: Option<RuleFeaturesCondition>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct RuleOsCondition {
	pub name: Option<Os>,
	pub version: Option<String>,
	pub arch: Option<Arch>,
}

pub type RuleFeaturesCondition = HashMap<String, bool>;

#[derive(Debug, Deserialize)]
pub struct Rule {
	pub action: RuleAction,
	#[serde(flatten)]
	pub condition: Condition,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Artifact {
	pub path: PathBuf,
	pub sha1: String,
	pub size: i32,
	pub url: Url,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LibraryDownloadEntry {
	pub artifact: Artifact,
}

#[derive(Debug, Deserialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum Library {
	Custom {
		name: String,
		url: Url,
	},
	Native {
		downloads: LibraryDownloadEntry,
		name: String,
		#[serde(flatten)]
		rules: Option<Vec<Rule>>,
		classifiers: HashMap<String, Artifact>,
		natives: HashMap<Os, String>,
	},
	Seminative {
		downloads: LibraryDownloadEntry,
		name: String,
		#[serde(flatten)]
		rules: Vec<Rule>,
	},
	Default {
		downloads: LibraryDownloadEntry,
		name: String,
	},
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetIndexResource {
	pub id: String,
	pub sha1: String,
	pub size: i32,
	pub total_size: i32,
	pub url: Url,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PackageDownloads {
	pub client: Resource,
	pub client_mappings: Resource,
	pub server: Resource,
	pub server_mappings: Resource,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
	pub sha1: String,
	pub size: i32,
	pub url: Url,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JavaVersion {
	pub component: String,
	pub major_version: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientLogging {
	pub argument: String,
	#[serde(rename = "type")]
	pub log_type: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Logging {
	pub client: ClientLogging,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VersionType {
	Release,
	Snapshot,
	OldBeta,
	OldAlpha,
}

#[derive(Debug, Deserialize)]
pub struct Args {
	pub game: Vec<Argument>,
	pub jvm: Vec<Argument>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum ArgsContainer {
	Modern {
		arguments: Args,
	},
	Legacy {
		minecraft_arguments: String,
	}
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Manifest {
	pub inherits_from: Option<String>,
	#[serde(flatten)]
	pub arguments: ArgsContainer,
	pub asset_index: AssetIndexResource,
	pub assets: String,
	pub downloads: PackageDownloads,
	pub id: String,
	pub java_version: JavaVersion,
	pub libraries: Vec<Library>,
	pub logging: Logging,
	pub main_class: String,
	pub release_time: String,
	pub time: String,
	#[serde(rename = "type")]
	pub version_type: VersionType,
}
