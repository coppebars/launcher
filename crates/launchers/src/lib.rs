#![feature(async_fn_in_trait)]

pub mod distros;
pub mod launch;
pub mod specs;
pub mod error;
pub mod api;
pub mod jre;

pub use error::Error;
