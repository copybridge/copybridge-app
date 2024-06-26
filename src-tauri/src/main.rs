// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use config_file::Config;
// use config::Clipboard;
use tauri::{AppHandle, Manager};
// use anyhow::{Result, Context};
mod cli;
mod config_file;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn read_config() -> Result<Config, String> {
    match config_file::read() {
        Ok(config) => Ok(config),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
fn write_config(config: Config) -> Result<(), String> {
    match config_file::write(&config) {
        Ok(()) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

fn main() {
    let mut config = config_file::read().unwrap();
    println!("{:#?}", config);
    config.server = "localhost:8080".to_string();
    // config.clipboards.push(Clipboard {
    //     id: 100002,
    //     name: "test".to_string(),
    //     is_encrypted: false,
    //     password: None,
    // });
    // config_file::write(&config).unwrap();
    
    cli::handle_cli();

    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let _ = show_window(app);
        }))
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![read_config])
        .invoke_handler(tauri::generate_handler![write_config])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    // app.run(|_app_handle, event| match event {
    //     // tauri::RunEvent::ExitRequested { api, .. } => {
    //     //     api.prevent_exit();
    //     // }
    //     _ => {}
    // })

}

fn show_window(app: &AppHandle) {
    let windows = app.webview_windows();

    windows
        .values()
        .next()
        .expect("Sorry, no window found")
        .set_focus()
        .expect("Can't Bring Window to Focus");
}
