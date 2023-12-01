pub mod action;
pub mod executor;

pub trait Install {
	type Error;

	async fn install(self) -> Result<Vec<action::Action>, Self::Error>;
}
