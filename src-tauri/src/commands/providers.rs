#[allow(unused)]
#[tauri::command]
pub async fn list() -> Result<[String; 1], ()> {
	Ok(["Hello".into()])
}
