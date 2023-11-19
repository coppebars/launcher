use std::path::Path;

mod versions;

#[tokio::main]
async fn main() {
	let result = versions::lookup_versions(Path::new("./minecraft")).await.unwrap();

	println!("{:#?}", &result);
}
