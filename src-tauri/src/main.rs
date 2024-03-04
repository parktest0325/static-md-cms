// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod setting;
mod ssh;

use anyhow::Result;
use setting::{load_config, save_config};

fn main() -> Result<()> {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![load_config, save_config])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
