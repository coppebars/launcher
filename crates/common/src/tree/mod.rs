use std::path::{
	Path,
	PathBuf,
};

pub struct CanonicalRoot<'a> {
	pub root: &'a Path,
	pub id: &'a str,
	pub jre: &'a str,
	pub assets: &'a str,
}

pub struct CanonicalTree {
	pub libraries_dir: PathBuf,
	pub version_dir: PathBuf,
	pub version_natives_dir: PathBuf,
	pub jre_dir: PathBuf,
	pub assets_dir: PathBuf,
	pub assets_indexes_dir: PathBuf,
	pub assets_objects_dir: PathBuf,
}

impl<'a> From<CanonicalRoot<'a>> for CanonicalTree {
	fn from(value: CanonicalRoot<'a>) -> Self {
		let version_dir = value.root.join("versions").join(value.id);
		let assets_dir = value.root.join("assets");

		Self {
			libraries_dir: value.root.join("libraries"),
			version_dir: version_dir.clone(),
			version_natives_dir: version_dir.join("natives"),
			jre_dir: value.root.join("jre").join(value.jre),
			assets_indexes_dir: assets_dir.join("indexes"),
			assets_objects_dir: assets_dir.join("objects"),
			assets_dir,
		}
	}
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum CanonicalKind {
	Library,
	AssetsObject,
	AssetsIndex,
	Version,
	VersionNative,
	Jre,
}

impl CanonicalTree {
	pub fn for_kind(&self, kind: CanonicalKind) -> PathBuf {
		match kind {
			CanonicalKind::Library => self.libraries_dir.clone(),
			CanonicalKind::AssetsObject => self.assets_objects_dir.clone(),
			CanonicalKind::AssetsIndex => self.assets_indexes_dir.clone(),
			CanonicalKind::Version => self.version_dir.clone(),
			CanonicalKind::VersionNative => self.version_natives_dir.clone(),
			CanonicalKind::Jre => self.jre_dir.clone(),
		}
	}
}
