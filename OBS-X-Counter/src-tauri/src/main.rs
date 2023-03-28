// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyEvent, GlobalHotKeyManager,
};
use tauri::Manager;

// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    // initialize the hotkeys manager
    let manager = GlobalHotKeyManager::new().unwrap();

    // construct the hotkey
    let hotkey = HotKey::new(Some(Modifiers::SHIFT), Code::ArrowUp);

    // register it
    manager.register(hotkey).unwrap();

    tauri::Builder::default()
        .setup(|app| {
            // app.emit_all(
            //     "key-registered",
            //     Payload {
            //         message: "key was pressed".into(),
            //     },
            // )
            // .unwrap();

            if let Ok(event) = GlobalHotKeyEvent::receiver().try_recv() {
                println!("{:?}", event);
                app.emit_all(
                    "key-registered",
                    Payload {
                        message: "key was pressed".into(),
                    },
                ).unwrap();
            }
            // via rust syntax, this last line is the return value technically
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
