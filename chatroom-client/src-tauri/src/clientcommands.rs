use std::{fmt::Debug, io, net::SocketAddr, str::FromStr};
use tauri::{AppHandle, Emitter, State};
use tokio::sync::Mutex;

use crate::{
    clientstate::{ClientState, ConnectionThreadCommand},
    consts::EventNames,
    errors::CommandStatusError,
    packethelper::PacketH,
    utils::IoResult,
};

#[tauri::command]
pub async fn connect(
    ip: String,
    username: String,
    app: AppHandle,
    state: State<'_, Mutex<ClientState>>,
) -> Result<(), String> {
    app.emit(EventNames::STATUS_UPD, "Connecting...".to_string())
        .map_err(|e| "Couldn't emit to app".to_string())?;
    if let Ok(addr) = ip.parse::<SocketAddr>() {
        let rx = state.lock().await.init_channel();
        println!("Starting connect from attempting task.");
        ClientState::connect(rx, addr, username, app).await;
    } else {
        return Err(CommandStatusError::InvalidIp.to_string());
    }

    println!("Connected to {ip}, with a username from React!");

    Ok(())
}

#[tauri::command]
pub async fn disconnect(state: State<'_, Mutex<ClientState>>) -> Result<(), String> {
    let guard = state.lock().await;

    if let Some(tx) = guard.tx.clone() {
        tx.send(ConnectionThreadCommand::Disconnect)
            .await
            .map_err(|e| e.to_string())
    } else {
        Err(CommandStatusError::NoTransmitter.to_string())
    }
}

#[tauri::command]
pub async fn send(message: String, state: State<'_, Mutex<ClientState>>) -> Result<(), String> {
    println!("Attempting to send message.");

    let guard = state.lock().await;

    if let Some(tx) = guard.tx.clone() {
        tx.send(ConnectionThreadCommand::Packet(PacketH::message_nt(
            message,
        )))
        .await
        .map_err(|e| e.to_string())
    } else {
        Err(CommandStatusError::NoTransmitter.to_string())
    }
}
