mod api;

#[tokio::main]
async fn main() {
	let manifest = api::get_manifest("75c25311ac5098c9a5c0b7747e932537752243f0", "23w43b").await;

	println!("{:#?}", manifest);
}
