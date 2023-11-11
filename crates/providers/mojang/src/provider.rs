use pvdr_common::Provider;

pub struct Mojang;

impl Provider for Mojang {
	fn name(&self) -> &'static str {
		"mojang"
	}
}

pub const MOJANG: Mojang = Mojang;
