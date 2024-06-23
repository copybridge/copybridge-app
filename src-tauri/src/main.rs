// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process;
use tauri::App;
use tauri_plugin_cli::CliExt;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_cli::init())
        // .plugin(tauri_plugin_single_instance::init(|_app, _args, _cwd| {
        //     // if args.len() < 2 {
        //     //     let _ = show_window(app);
        //     // }
        // }))
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_shell::init())
        // .setup(|app| {
        //     let _ = handle_cli(app);
        //     Ok(())
        // })
        .invoke_handler(tauri::generate_handler![greet])
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    let _ = handle_cli(&app);

    app.run(|_app_handle, event| match event {
        // tauri::RunEvent::ExitRequested { api, .. } => {
        //     api.prevent_exit();
        // }
        _ => {}
    })

}

// fn show_window(app: &App) {
//     let windows = app.webview_windows();

//     windows
//         .values()
//         .next()
//         .expect("Sorry, no window found")
//         .set_focus()
//         .expect("Can't Bring Window to Focus");
// }

fn handle_cli(app: &App) {
    match app.cli().matches() {
        Ok(matches) => {
            // println!("CLI matches: {:?}", matches);
            // process::exit(0);

            if matches.args.contains_key("help") {
                print!(
                    "{}",
                    matches.args.get("help").unwrap().value.as_str().unwrap()
                );
                // app.cleanup_before_exit();
                process::exit(0);
            }
        }
        Err(_) => {}
    }
}