// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod methods;
use methods::webserver::run_server;
use tauri::Manager;
// use methods::webserver;
#[tokio::main]
async fn main() {
  run_server();

  tauri::Builder::default()
    .run(tauri::generate_context!())
    .expect("error while running tauri application");

}

