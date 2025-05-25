// this file contains code defining Packet enums, and the code for their serializing and
// deserializing.

// todos are getting fixed later, just want to make the server to work (generally) first.
// won't say no to PR's fixing todos tho!

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::io::{self, ErrorKind};
use std::io::{Read, Write};
use std::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream as TokioTcpStream;

/// Packets sent from the Client, to the Server. (Serverbound)
#[derive(Deserialize, Serialize, Clone)]
pub enum ServerboundPacket {
    // TODO: Add RSA handshake.
    Login { username: String },
    // no username bundled for more customization on the server.
    Message { token: String, message: String },
    Heartbeat { token: String },
    Disconnect { token: String },
}

#[derive(Clone)]
pub enum ServerboundPacketNonToken {
    Login { username: String },
    Message { message: String },
    Heartbeat,
    Disconnect,
}

/// Packets sent from the Server, to the Client. (Clientbound)
#[derive(Serialize, Deserialize, Clone)]
pub enum ClientboundPacket {
    // TODO: Add RSA Handshake response.
    Token { token: String },
    Message { content: String },
    HeartbeatReq,
    Kicked { reason: String },
}

#[derive(Deserialize, Serialize, Clone)]
pub enum Packet {
    Clientbound(ClientboundPacket),
    Serverbound(ServerboundPacket),
}

#[async_trait]
pub trait AsyncPacketHandler {
    async fn handle_packet(&mut self, packet: Packet) -> io::Result<()>;
}

// Packet Recieve and Transmitter Traits. should add timeouts in the future.
#[async_trait]
pub trait AsyncPacketRT {
    async fn send_packet(&mut self, packet: Packet) -> io::Result<()>;
    async fn receive_packet(&mut self) -> io::Result<Packet>;
}

pub trait SyncPakcetRT {
    fn send_packet(&mut self, packet: Packet) -> io::Result<()>;
    fn receive_packet(&mut self) -> io::Result<Packet>;
}

impl SyncPakcetRT for TcpStream {
    fn send_packet(&mut self, packet: Packet) -> io::Result<()> {
        let bytes = bincode::serialize(&packet).map_err(|e| io::Error::new(ErrorKind::Other, e))?;
        let len = bytes.len() as u32;

        self.write_all(&len.to_be_bytes())?;
        self.write_all(&bytes)?;

        Ok(())
    }

    fn receive_packet(&mut self) -> io::Result<Packet> {
        let mut len_buf = [0u8; 4]; // u32, since 4 * 4 * 4 * 4 = 32
        self.read_exact(&mut len_buf)?;
        let len = u32::from_be_bytes(len_buf);

        let mut packet_buf = vec![0u8; len as usize];
        self.read_exact(&mut packet_buf)?;

        let packet: Packet =
            bincode::deserialize(&packet_buf).map_err(|e| io::Error::new(ErrorKind::Other, e))?;

        Ok(packet)
    }
}

#[async_trait]
impl AsyncPacketRT for TokioTcpStream {
    async fn send_packet(&mut self, packet: Packet) -> io::Result<()> {
        let bytes = bincode::serialize(&packet).map_err(|e| io::Error::new(ErrorKind::Other, e))?;
        let len = bytes.len() as u32;

        self.write_all(&len.to_be_bytes()).await?;
        self.write_all(&bytes).await?;

        Ok(())
    }

    // TODO: Fix edge cases for reading and length
    async fn receive_packet(&mut self) -> io::Result<Packet> {
        let mut len_buf = [0u8; 4]; // u32, since 4 * 4 * 4 * 4 = 32
        self.read_exact(&mut len_buf).await?;
        let len = u32::from_be_bytes(len_buf);

        let mut packet_buf = vec![0u8; len as usize];
        self.read_exact(&mut packet_buf).await?;

        let packet: Packet =
            bincode::deserialize(&packet_buf).map_err(|e| io::Error::new(ErrorKind::Other, e))?;

        Ok(packet)
    }
}
