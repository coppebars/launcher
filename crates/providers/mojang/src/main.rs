mod api;
mod install;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let items = install::get_items("1.20.1").await?;

	
	
  Ok(())
}
