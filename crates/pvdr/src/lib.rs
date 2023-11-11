pub trait Provider {
	fn name(&self) -> &'static str;
}
