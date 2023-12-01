use std::path::Path;

#[tauri::command]
pub async fn lookup_versions(path: &Path) -> Result<Vec<lookup::VersionOverview>, String> {
	lookup::lookup_versions(path)
		.await
		.map_err(|err| err.to_string())
}
