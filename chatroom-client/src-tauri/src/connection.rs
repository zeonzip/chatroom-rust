use crate::clientboundpackethandler::{ClientboundPacketHandler, PacketOutcome};
use shared::extensions::ConnectTimeoutAsync;
use shared::packet::{AsyncPacketHandler, AsyncPacketRT, ClientboundPacket, Packet};
use std::{io, net::SocketAddr, str::FromStr, time::Duration};
use tauri::{AppHandle, Emitter};
use tokio::io::AsyncWriteExt;

use tokio::net::TcpStream;

use crate::consts::EventNames;
use crate::{errors::ClientNetworkError, packethelper::PacketH};

const CONNECTTIMEOUT: Duration = Duration::from_secs(5);

pub struct Connection {
    stream: TcpStream,
    token: String,
}

impl Connection {
    pub async fn connect(addr: SocketAddr, username: String) -> io::Result<Connection> {
        let mut stream = TcpStream::connect_timeout(&addr, CONNECTTIMEOUT).await?;
        stream.send_packet(PacketH::login(username)).await?;

        // next expected packet is a token, because of our protocol
        if let Packet::Clientbound(ClientboundPacket::Token { token }) =
            stream.receive_packet().await?
        {
            Ok(Connection { stream, token })
        } else {
            Err(ClientNetworkError::UnexpectedPacket.into())
        }
    }

    pub fn get_token(&self) -> &str {
        &self.token
    }

    pub async fn send_packet(&mut self, packet: Packet) -> io::Result<()> {
        self.stream.send_packet(packet).await
    }

    pub async fn receive_packet(&mut self) -> io::Result<Packet> {
        self.stream.receive_packet().await
    }

    pub async fn close(mut self) -> io::Result<()> {
        self.send_packet(PacketH::disconnect(self.token.clone()))
            .await?;
        self.stream.shutdown().await
    }
}

impl Connection {
    // custom async handler due to &mut AppHandle
    pub async fn handle_packet(
        &mut self,
        packet: Packet,
        app: &mut AppHandle,
    ) -> io::Result<PacketOutcome> {
        match packet {
            Packet::Clientbound(mut cbpacket) => Ok(cbpacket.handle_packet(app, self).await?),
            _ => return Err(ClientNetworkError::UnimplementedPacket.into()),
        }
    }
}
