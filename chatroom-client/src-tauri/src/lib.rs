mod clientboundpackethandler;
mod clientcommands;
mod clientstate;
mod connection;
mod consts;
mod errors;
mod packethelper;
mod utils;

use clientstate::ClientState;
use tauri::{generate_handler, Manager};
use tokio::sync::Mutex;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(Mutex::new(ClientState::new()))
        .invoke_handler(generate_handler![
            clientcommands::connect,
            clientcommands::disconnect,
            clientcommands::send,
        ])
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
