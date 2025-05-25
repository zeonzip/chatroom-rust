use crate::errors::ServerNetworkError;
use crate::server::ChatroomServer;
use crate::user::UserId;
use crate::user::shareduser::{CrossUserCommand, Feedback, SharedUser};
use crate::user::user::User;
use shared::packet::{AsyncPacketRT, ClientboundPacket, Packet, ServerboundPacket};
use std::io;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::select;
use tokio::sync::Mutex;

pub struct ClientHandler {
    stream: TcpStream,
}

impl ClientHandler {
    pub fn new(stream: TcpStream) -> Self {
        Self { stream }
    }
    pub async fn handle(mut self, server: Arc<Mutex<ChatroomServer>>) -> io::Result<()> {
        let packet = self.stream.receive_packet().await?;
        let user;
        let mut dp;

        println!("Attempting login...");
        match packet {
            Packet::Serverbound(p) => match p {
                ServerboundPacket::Login { username } => {
                    let id = (server.lock().await.users.len() + 1) as UserId;
                    user = Arc::new(Mutex::new(
                        User::connect(username.clone(), id, self.stream).await?,
                    ));
                    let (dpt, su) = SharedUser::new(username, id);
                    server.lock().await.users.insert(id, su);
                    dp = dpt;

                    println!("Login successfull!")
                }
                _ => return Err(ServerNetworkError::InvalidPacketFromClient.into()),
            },
            Packet::Clientbound(_) => {
                self.stream
                    .send_packet(Packet::Clientbound(ClientboundPacket::Kicked {
                        reason: String::from("Sent clientbound packet to server"),
                    }))
                    .await?;
                return Err(ServerNetworkError::SentClientboundToServer.into());
            }
        }

        println!("Starting user loop.");

        loop {
            let (mut eoptcommand, mut eresresult) = (None, None);

            {
                select! {
                    biased;

                    resresult = {
                        let user = user.clone();
                        async move {
                            let mut guard = user.lock().await;
                            guard.receive_packet().await
                        }
                    } => {
                        println!("Received packet from user!");
                        eresresult = Some(resresult);
                    }

                    optcommand = dp.recv() => {
                        println!("Received command!");
                        eoptcommand = Some(optcommand);
                    }
                }
            }

            if let Some(Ok(result)) = eresresult {
                println!("Dispatching packet.");
                match result {
                    Packet::Serverbound(packet) => {
                        ChatroomServer::handle_packet(packet, server.clone(), user.clone()).await?
                    }
                    _ => return Err(ServerNetworkError::SentClientboundToServer.into()),
                }
            }

            if let Some(Some(command)) = eoptcommand {
                println!("Dispatching command!");

                match command {
                    CrossUserCommand::Packet(p) => {
                        match dp
                            .send(CrossUserCommand::Feedback(Feedback::PacketSendResult(
                                user.lock().await.send_packet(p).await,
                            )))
                            .await
                        {
                            Err(_) => break,
                            _ => {}
                        }
                    }
                    CrossUserCommand::Disconnect => break,
                    CrossUserCommand::Kick(reason) => {
                        user.lock()
                            .await
                            .send_packet(Packet::Clientbound(ClientboundPacket::Kicked { reason }))
                            .await?;
                        break;
                    }
                    _ => {}
                }
            }
        }

        server.lock().await.users.remove(&user.lock().await.id);

        Ok(())
    }
}
