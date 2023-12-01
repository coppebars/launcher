mod base;
pub mod utils;

pub use base::*;

pub trait TryIntoLauncher {
	type Error;

	fn try_into_launcher(self) -> Result<ProcessLauncher, Self::Error>;
}
