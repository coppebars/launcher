use {
	crate::error::IpcError,
	std::path::Path,
};

#[tauri::command]
pub async fn lookup_versions(path: &Path) -> Result<Vec<lookup::VersionOverview>, IpcError> {
	Ok(lookup::lookup_versions(path).await?)
}
