pub mod piston;
pub mod launchermeta;
pub mod resources;

pub(crate) mod prelude {
	pub(crate) use {
		super::url,
		serde::Deserialize,
		std::{
			collections::HashMap,
			slice::Iter,
			vec::IntoIter,
		},
		url::Url,
	};
}

#[allow(unused_imports)]
use url::Url;

pub(crate) macro url($($url:tt)+) {
	Url::parse(&format!($($url)+)).unwrap()
}
