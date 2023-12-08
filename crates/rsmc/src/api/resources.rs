use super::prelude::*;

pub fn make_url(hash: &str) -> Url {
	let short = &hash[..2];

	url!("https://resources.download.minecraft.net/{short}/{hash}")
}
