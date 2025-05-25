/*
 TO ANYONE READING THIS CODE:
 This is code I've been working on while learning rust and
 I've realized after reading over that it's messy, unpredictable
 lack associaton and abstraction.

 Please don't judge!
*/

use crate::client::clienthandler::ClientHandler;
use crate::errors::ServerNetworkError;
use crate::user::UserId;
use crate::user::shareduser::{CrossUserCommand, SharedUser};
use crate::user::user::User;
use shared::packet::{ClientboundPacket, Packet, ServerboundPacket};
use std::collections::HashMap;
use std::io;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Mutex;

pub struct ChatroomServer {
    pub users: HashMap<UserId, SharedUser>,
}

impl ChatroomServer {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
        }
    }

    pub async fn run(server: Arc<Mutex<Self>>, listener: TcpListener) -> io::Result<()> {
        while let (stream, _) = listener.accept().await? {
            println!("New stream connected!");
            tokio::spawn(ClientHandler::new(stream).handle(server.clone()));
        }

        Ok(())
    }

    pub async fn global_send(
        packet: ClientboundPacket,
        server: Arc<Mutex<ChatroomServer>>,
    ) -> io::Result<()> {
        println!("Attempting global packet send!");

        let packet = Packet::Clientbound(packet);

        let mut guard = server.lock().await;
        let mut to_disconnect = Vec::new();

        println!("Starting loop!");
        println!("SharedUsers: {}", guard.users.len());
        for user in &mut guard.users.values_mut() {
            println!("Found user to send to!");
            if user
                .dp
                .send(CrossUserCommand::Packet(packet.clone()))
                .await
                .is_err()
            {
                println!("User is dead, and is set for cleanup.");
                to_disconnect.push(user.id);
            }
        }
        println!("Ended loop!");

        for id in to_disconnect {
            guard.users.remove(&id);
            println!("Disposed dead user.")
        }

        Ok(())
    }

    pub async fn handle_packet(
        packet: ServerboundPacket,
        server: Arc<Mutex<ChatroomServer>>,
        user: Arc<Mutex<User>>,
    ) -> io::Result<()> {
        println!("Server received dispatch, analyzing...");
        match packet {
            ServerboundPacket::Message { token, message } => {
                let mut guard = user.lock().await;

                println!("Running message packet.");
                if guard.token == token {
                    println!("Valid token!");
                    println!("Attempting to globally propagate...");
                    Self::global_send(
                        ClientboundPacket::Message {
                            content: format!("[{}]: {}", guard.username.clone(), message),
                        },
                        server,
                    )
                    .await?;
                    println!("globally propagated message.");
                } else {
                    println!("Kicking user for invalid token.");
                    user.lock().await.kick("Used invalid token!").await?;
                    println!("Kicked user for invalid token!");
                }
            }
            ServerboundPacket::Heartbeat { .. } => {
                // no function yet (working on heartbeats)
            }
            ServerboundPacket::Disconnect { .. } => {}
            _ => return Err(ServerNetworkError::InvalidPacketFromClient.into()),
        }

        Ok(())
    }
}
