#! [cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

use tauri::Manager;
use std::env;

fn main() {
	if cfg!(windows) {
		tauri::Builder::default()
			.invoke_handler(tauri::generate_handler![get_input, show_main_window])
			.setup(|app| {
				let windows = app.get_window(&"main").unwrap();
				window_shadows::set_shawdow(&window, true).expect("Unsupported platform!")
				Ok(())
			})
			.run(tauri::generate_context!())
			.expect("Error while running Nexus.")
	} else if cfg!(unix) {
		tauri::Builder::default()
			.invoke_handler(tauri::generate_handler![get_input, show_main_window])
			.run(tauri::generate_context!())
			.expect("Error while running Nexus.")
	} else if cfg!(target_os = "macos") {
		tauri::Builder::default()
		.invoke_handler(tauri::generate_handler![get_input, show_main_window])
		.run(tauri::generate_context!())
		.expect("Error while running Nexus.")
	}
}

#[tauri::command]
fn get_input() -> Vec<String> {
	let args = env::args().collect();
	args 
}
#[tauri::command]
async fn show_main_window(window: tauri::Window) {
	window.get_window("main").unwrap().show().unwrap()
}