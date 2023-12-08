use super::prelude::*;

pub mod runtime {
	use {
		super::*,
		std::fmt::{
			Debug,
			Display,
			Formatter,
		},
	};

	macro string_enum($name:ident { $($variant:ident = $str:literal,)* }) {
		#[derive(Debug, Deserialize, PartialEq, Eq, Hash)]
		pub enum $name {
			$(
				#[serde(rename = $str)]
				$variant,
			)*
		}

		impl Display for $name {
			fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
				match self {
					$(
						Self::$variant => write!(f, "{}", $str),
					)*
				}
			}
		}

		#[allow(clippy::should_implement_trait)]
		impl $name {
			pub fn from_str(value: &str) -> Option<Self> {
				match value {
					$(
						$str => Some(Self::$variant),
					)*
					_ => None
				}
			}
		}
	}

	string_enum! {
		Target {
			GameCore = "gamecore",
			Linux = "linux",
			LinuxI386 = "linux-i386",
			Macos = "mac-os",
			MacosArm64 = "mac-os-arm64",
			WindowsArm64 = "windows-arm64",
			WindowsX64 = "windows-x64",
			WindowsX86 = "windows-x86",
		}
	}

	string_enum! {
		ComponentType {
			JavaRuntimeAlpha = "java-runtime-alpha",
			JavaRuntimeBeta = "java-runtime-beta",
			JavaRuntimeGamma = "java-runtime-gamma",
			JavaRuntimeGammaSnapshot = "java-runtime-gamma-snapshot",
			JreLegacy = "jre-legacy",
			MinecraftJavaExe = "minecraft-java-exe",
		}
	}

	#[derive(Debug, Deserialize)]
	pub struct Resource {
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
		pub manifest: Resource,
		pub version: Version,
	}

	pub type Response = HashMap<Target, Component>;

	pub const URL: Url = url!("https://launchermeta.mojang.com/v1/products/java-runtime/2ec0cc96c44e5a76b9c8b7c39df7210883d12871/all.json");

	pub async fn get() -> Result<Response, reqwest::Error> {
		reqwest::get(URL).await?.json().await
	}
}
