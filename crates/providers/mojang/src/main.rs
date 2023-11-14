mod api;
mod install;

use common::manifest::{AssetIndex, Manifest};
use install::Install;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let manifest = api::get_manifest("75c25311ac5098c9a5c0b7747e932537752243f0", "23w43b").await?;

	let manifest = match manifest {
		Manifest::Root(it) => it,
		_ => unreachable!()
	};

	let asset_index = reqwest::get(manifest.asset_index.url.clone()).await?.json::<AssetIndex>().await?;

  let items = asset_index.into_items();

  println!("{:#?}", items);

	Ok(())
}
