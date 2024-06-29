// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use config_file::{read_config, write_config};
// use config::Clipboard;
use tauri::{AppHandle, Manager};
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
};
// use anyhow::{Result, Context};
mod cli;
mod config_file;
mod cmd;
mod err;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    // let config = config_file::read().unwrap();
    // println!("{:#?}", config);
    // config.server = ;
    // config.clipboards.push(Clipboard {
    //     id: 100002,
    //     name: "test".to_string(),
    //     is_encrypted: false,
    //     password: None,
    // });
    // config_file::write(&config).unwrap();

    cli::handle_cli();

    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let _ = show_window(app);
        }))
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            read_config,
            write_config,
            cmd::create,
            cmd::add,
            cmd::get_content,
            cmd::set_content,
            cmd::remove,
            cmd::delete,
        ])
        .on_window_event(|window, event| match event {
            // tauri::WindowEvent::Focused(focused) => {
            //     // hide window whenever it loses focus
            //     if !focused {
            //         window.hide().unwrap();
            //     }
            // }
            tauri::WindowEvent::CloseRequested { api, .. } => {
                window.hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .setup(|app| {
            let toggle = MenuItemBuilder::with_id("toggle", "Toggle Window").build(app)?;
            let quit = MenuItemBuilder::with_id("quit", "Quit").build(app)?;
            let menu = MenuBuilder::new(app).items(&[&toggle, &quit]).build()?;
            let _tray = TrayIconBuilder::new()
                .menu(&menu)
                .on_menu_event(move |app, event| match event.id().as_ref() {
                    "toggle" => {
                        toggle_window(app);
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => (),
                })
                .menu_on_left_click(false)
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                            button: MouseButton::Left,
                            button_state: MouseButtonState::Up,
                            ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(webview_window) = app.get_webview_window("main") {
                        let _ = webview_window.show();
                        let _ = webview_window.set_focus();
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    // app.run(|_app_handle, event| match event {
    //     tauri::RunEvent::ExitRequested { api, .. } => {
    //         event.window().hide().unwrap();
    //         api.prevent_exit();
    //     }
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

fn toggle_window(app: &AppHandle) {
    let windows = app.webview_windows();

    let my_window = windows
            .values()
            .next()
            .expect("Sorry, no window found");
    
    if my_window.is_visible().expect("Can't Check Window Visibility") {
        let _ = my_window.hide();
    } else {
        let _ = my_window.show();
    }
}