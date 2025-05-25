use super::UserId;
use crate::consts::TOKEN_LEN;
use rand::distr::{Alphanumeric, SampleString};
use shared::packet::{AsyncPacketRT, ClientboundPacket, Packet};
use std::io;
use tokio::net::TcpStream;

pub struct User {
    pub username: String,
    pub id: UserId,
    pub token: String,
    stream: TcpStream,
    // RSA
}

impl User {
    pub async fn connect(username: String, id: UserId, mut stream: TcpStream) -> io::Result<User> {
        let token = Self::gen_token();
        let username = username;
        stream
            .send_packet(Packet::Clientbound(ClientboundPacket::Token {
                token: token.clone(),
            }))
            .await?;

        Ok(User {
            username,
            id,
            token,
            stream,
        })
    }

    pub async fn kick(&mut self, reason: &str) -> io::Result<()> {
        self.stream
            .send_packet(Packet::Clientbound(ClientboundPacket::Kicked {
                reason: reason.to_string(),
            }))
            .await
    }

    pub async fn send_packet(&mut self, packet: Packet) -> io::Result<()> {
        self.stream.send_packet(packet).await
    }

    pub async fn receive_packet(&mut self) -> io::Result<Packet> {
        self.stream.receive_packet().await
    }
}

// user utils
impl User {
    // worth noting this isn't crypto safe, but it's good enough for this project.
    pub fn gen_token() -> String {
        Alphanumeric.sample_string(&mut rand::rng(), TOKEN_LEN)
    }
}

// no username bundled for more customization on the server.
struct Message {
    content: String,
}
