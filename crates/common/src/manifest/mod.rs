use std::collections::HashSet;
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RuleAction {
  Allow,
  Disallow,
}

#[derive(Debug, Deserialize)]
pub struct Condition {
  pub os: Option<RuleOsCondition>,
  pub features: Option<RuleFeaturesCondition>,
}

#[derive(Debug, Deserialize)]
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

impl Rule {
	fn check_os_condition(&self) -> bool {
		let mut allow = true;

		if let Some(os_condition) = &self.condition.os {
			if let Some(os_name) = &os_condition.name {
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

			if let Some(os_arch) = &os_condition.arch {
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

		allow
	}

	pub fn unwrap_featured(&self, features: HashSet<&str>) -> bool {
		let mut allow = self.check_os_condition();

		if let Some(features_condition) = &self.condition.features {
			allow = features_condition.keys().all(|it| features.contains(it.as_str()));
		}

		match self.action {
			RuleAction::Allow => allow,
			RuleAction::Disallow => !allow,
		}
	}

	pub fn unwrap(&self) -> bool {
		let allow = self.check_os_condition();

		match self.action {
			RuleAction::Allow => allow,
			RuleAction::Disallow => !allow,
		}
	}
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Artifact {
  pub path: PathBuf,
  pub sha1: String,
  pub size: u64,
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
    rules: Vec<Rule>,
    classifiers: HashMap<String, Artifact>,
    natives: HashMap<Os, String>,
  },
  Seminative {
    downloads: LibraryDownloadEntry,
    name: String,
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
  pub size: u64,
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
pub struct ModernArgs {
  arguments: Args,
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

#[derive(Debug, Deserialize)]
pub struct LegacyArgs {
  #[serde(rename = "minecraft_arguments")]
  arguments: String,
}

impl From<LegacyArgs> for ModernArgs {
	fn from(value: LegacyArgs) -> Self {
		value.arguments.into()
	}
}

#[derive(Debug, Deserialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum ArgsContainer {
  Modern(ModernArgs),
  Legacy(LegacyArgs),
}

impl ArgsContainer {
  fn merge(self, with: ArgsContainer) -> Self {
    use ArgsContainer::*;

		match with {
			Modern(ModernArgs { arguments }) => {
				let Args { jvm: jvm_ext, game: game_ext } = arguments;
				let mut modern = self.into_modern();
				let Args { ref mut jvm, ref mut game } = modern.arguments;

				jvm.extend(jvm_ext);
				game.extend(game_ext);

				Modern(modern)
			}
			Legacy(args) => self.merge(Modern(args.into()))
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RootManifest {
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InheritedManifest {
  pub inherits_from: Option<String>,
  #[serde(flatten)]
  pub arguments: ArgsContainer,
  pub libraries: Vec<Library>,
  pub main_class: String,
  pub release_time: String,
  pub time: String,
  #[serde(rename = "type")]
  pub version_type: VersionType,
  pub id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum Manifest {
  Root(Box<RootManifest>),
  Inherited(Box<InheritedManifest>),
}

impl InheritedManifest {
  pub fn into_root(self, mut root: RootManifest) -> RootManifest {
    macro_rules! copy {
   	 ($($field:ident),+) => {
			 $(
			 	root.$field = self.$field;
			 )+
    	};
		}

    copy! {
      id,
      time,
      release_time,
      main_class
    };

		root.arguments = root.arguments.merge(self.arguments);
		root.libraries.extend(self.libraries);

    root
  }
}

#[derive(Debug, Deserialize)]
pub struct AssetObject {
	pub hash: String,
	pub size: u64
}

#[derive(Debug, Deserialize)]
pub struct AssetIndex {
	pub objects: HashMap<String, AssetObject>
}
