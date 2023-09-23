// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri_plugin_log::LogTarget;

#[tauri::command]
async fn open_window(handle: tauri::AppHandle) {
    let docs_window = tauri::WindowBuilder::new(
        &handle,
        "external",
        tauri::WindowUrl::External("https://tauri.app/".parse().unwrap()),
    ).build().unwrap();

    docs_window.set_title("Add todo").unwrap();
    docs_window.set_resizable(false).unwrap();
    docs_window.set_always_on_top(true).unwrap();
    docs_window.set_minimizable(false).unwrap();
}

fn main() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([LogTarget::LogDir, LogTarget::Stdout, LogTarget::Webview])
                .build(),
        )
        .invoke_handler(tauri::generate_handler![open_window])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
