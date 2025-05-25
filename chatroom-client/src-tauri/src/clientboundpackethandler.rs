use std::io::{self, ErrorKind};

use async_trait::async_trait;
use shared::packet::ClientboundPacket;
use tauri::{AppHandle, Emitter};

use crate::{
    connection::Connection, consts::EventNames, errors::ClientNetworkError, packethelper::PacketH,
    utils::IoResult,
};

pub enum PacketOutcome {
    Continue,
    Exit,
    ExitWithoutCleanup,
}

#[async_trait]
pub trait ClientboundPacketHandler {
    async fn handle_packet(
        self,
        app: &mut AppHandle,
        conn: &mut Connection,
    ) -> io::Result<PacketOutcome>;
}

#[async_trait]
impl ClientboundPacketHandler for ClientboundPacket {
    async fn handle_packet(
        self,
        app: &mut AppHandle,
        conn: &mut Connection,
    ) -> io::Result<PacketOutcome> {
        println!("Receiving packet!");

        match self {
            ClientboundPacket::Message { content } => {
                app.emit(EventNames::MESSAGE_RECEIVED, content).to_io()?;
                Ok(PacketOutcome::Continue)
            }
            ClientboundPacket::Kicked { reason } => {
                app.emit(EventNames::KICKED, reason).to_io()?;
                Ok(PacketOutcome::ExitWithoutCleanup)
            }
            ClientboundPacket::Token { token } => Err(ClientNetworkError::UnexpectedPacket.into()),
            ClientboundPacket::HeartbeatReq => {
                conn.send_packet(PacketH::heartbeat(conn.get_token().to_string()))
                    .await?;
                Ok(PacketOutcome::Continue)
            }
            _ => Err(ClientNetworkError::UnimplementedPacket.into()),
        }
    }
}
