pub mod jre;
pub mod manifest;

pub mod prelude {
	pub use super::{
		jre::{
			Manifest as JreManifest,
			Entry as JreEntry,
			ComponentType as JreComponentType,
			Target as JreTarget,
		},
		manifest::{
			AssetIndex,
			IntoClasspath,
			JoinClasspath,
			Library,
			Manifest,
			ModernArgs,
			Os,
			Rule,
		},
	};
}
