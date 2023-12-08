#![feature(decl_macro)]

pub(crate) mod private;

#[cfg(feature = "api")]
pub mod api;
pub mod error;
#[cfg(feature = "launcher")]
pub mod launcher;
#[cfg(feature = "spec")]
pub mod spec;

pub(crate) use {
	error::Error,
	private::*,
};
