use {
  serde::Deserialize,
  std::collections::{
    HashMap,
    HashSet,
  },
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
}

#[derive(Debug, Deserialize)]
pub struct Manifest {
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
  pub manifest: Manifest,
  pub version: Version,
}

pub type JavaRuntime = HashMap<Target, HashSet<ComponentType, Component>>;
