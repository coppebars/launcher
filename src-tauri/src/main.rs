// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(target_os = "windows")]
use window_vibrancy::apply_acrylic;
#[cfg(target_os = "macos")]
use window_vibrancy::{
	apply_vibrancy,
	NSVisualEffectMaterial,
};

use {
	tauri::Manager,
	tracing::info,
	tracing_subscriber::{
		layer::SubscriberExt,
		util::SubscriberInitExt,
	},
};

fn main() {
	tracing_subscriber::registry()
		.with(tracing_subscriber::fmt::layer())
		.init();

	info!("Rslauncher v{}", env!("CARGO_PKG_VERSION"));

	std::env::set_var(
		"WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS",
		"--ignore-gpu-blocklist",
	);

	tauri::Builder::default()
		.plugin(tauri_plugin_window::init())
		.plugin(tauri_plugin_shell::init())
		.plugin(tauri_plugin_dialog::init())
		.plugin(tauri_plugin_updater::Builder::new().build())
		.on_window_event(|e| {
			if let tauri::WindowEvent::Resized(_) = e.event() {
				std::thread::sleep(std::time::Duration::from_nanos(1));
			}
		})
		.setup(|app| {
			#[allow(unused)]
			let window = app.get_window("main").unwrap();

			#[cfg(target_os = "macos")]
			apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, None)
				.expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

			#[cfg(target_os = "windows")]
			apply_acrylic(&window, None)
				.expect("Unsupported platform! 'apply_blur' is only supported on Windows");

			Ok(())
		})
		.invoke_handler(tauri::generate_handler![ipc::lookup_versions, ipc::launch,])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
