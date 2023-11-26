// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod hotkey;

use hotkey::InputManager;
use tauri::SystemTray;

fn main() {
    let manager = InputManager::new();
    manager.init();

    println!("build ing");

    let system_tray = SystemTray::new();

    tauri::Builder::default()
        .system_tray(system_tray)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
