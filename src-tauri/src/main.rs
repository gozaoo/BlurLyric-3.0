// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod methods;
use methods::webserver::run_server;
use tauri::Manager;
// use methods::webserver;
#[tokio::main]
async fn main() -> std::io::Result<()> {
  tauri::Builder::default()
    // .setup(setup::init)
    .run(tauri::generate_context!())
    .expect("error while running tauri application");

  run_server().await
}

