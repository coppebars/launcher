use pvdr_mojang::list_versions;

#[allow(unused)]
#[tauri::command]
pub async fn mojang_list_versions() -> Result<Vec<String>, String> {
	list_versions().await.map_err(|_| "request error".to_string())
}
