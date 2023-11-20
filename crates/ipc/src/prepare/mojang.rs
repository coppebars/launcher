use {
	crate::error::IpcError,
	pvdr_mojang::prepare,
	std::path::Path,
};

#[tauri::command]
pub async fn mojang_prepare(id: &str, path: &Path) -> Result<(), IpcError> {
	prepare(path, id).await?;

	Ok(())
}
