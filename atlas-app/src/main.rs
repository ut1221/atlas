// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use atlas::establish_connection_pool;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
mod exec;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let pool = establish_connection_pool();

    tauri::Builder::default()
        .manage(pool)
        .invoke_handler(tauri::generate_handler![greet, exec::update_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
