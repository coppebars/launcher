// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;

use tauri::Manager;
#[cfg(target_os = "windows")]
use window_vibrancy::apply_blur;

#[cfg(target_os = "macos")]
use window_vibrancy::{
  apply_vibrancy,
  NSVisualEffectMaterial,
};

fn main() {
  tauri::Builder::default()
		.plugin(tauri_plugin_window::init())
		.plugin(tauri_plugin_shell::init())
    .setup(|app| {
			#[allow(unused)]
      let window = app.get_window("main").unwrap();

      #[cfg(target_os = "macos")]
      apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, None)
        .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

      #[cfg(target_os = "windows")]
      apply_blur(&window, None)
        .expect("Unsupported platform! 'apply_blur' is only supported on Windows");

      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      commands::providers::mojang_list_versions
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
