mod api;
mod install;

pub fn id() -> &'static str {
	"mojang"
}

pub async fn list_versions() -> Result<Vec<String>, reqwest::Error> {
	let versions = api::get_versions_manifest().await?;

	Ok(versions.versions.into_iter().map(|it| it.id).collect())
}
