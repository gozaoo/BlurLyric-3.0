// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// mod setup;
use tauri::Manager;
// use methods::webserver::run_server;
fn main() {
  tauri::Builder::default()
    // .setup(setup::init)
    .run(tauri::generate_context!())
    .expect("error while running tauri application");

  // run_server()
}

