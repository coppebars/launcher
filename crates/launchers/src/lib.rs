#![feature(async_fn_in_trait)]

pub mod distros;
pub mod launch;
pub mod specs;

#[cfg(feature = "install")]
pub mod install;
