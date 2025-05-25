use serde::Serialize;
use shared::packet::{Packet, ServerboundPacket, ServerboundPacketNonToken};
use std::{
    io,
    net::{SocketAddr, TcpStream},
};
use tauri::{AppHandle, Emitter};
use tokio::{
    select,
    sync::mpsc::{channel, Receiver, Sender},
};

use crate::{
    clientboundpackethandler::PacketOutcome, connection::Connection, consts::EventNames,
    errors::ClientNetworkError, packethelper::PacketH,
};

pub enum ConnectionThreadCommand {
    Packet(ServerboundPacketNonToken),
    Disconnect,
}

#[derive(Debug, Serialize, Clone)]
pub enum ExitReason {
    Kicked,
    Disconnected,
    LostConnection,
}

pub struct ClientState {
    pub tx: Option<Sender<ConnectionThreadCommand>>,
}

#[derive(Debug)]
pub enum ConnectionOutcome {
    Exit,
    ExitWithoutCleanup,
    Err(io::Error),
}

impl ClientState {
    pub fn new() -> Self {
        Self { tx: None }
    }

    pub fn init_channel(&mut self) -> Receiver<ConnectionThreadCommand> {
        let (mut tx, mut rx) = channel(100);
        self.tx = Some(tx);
        rx
    }

    pub async fn connect(
        mut rx: Receiver<ConnectionThreadCommand>,
        addr: SocketAddr,
        username: String,
        mut app: AppHandle,
    ) {
        println!("Running utility starter...");
        tokio::spawn(async move {
            println!("Spawned task in utility starter...");
            let app2 = app.clone();
            println!("Starting start connection task");
            let res = Self::start_connection(rx, addr, username, app)
                .await
                .map_err(|e| e.to_string());
            app2.emit(EventNames::SERVER_RESULT, res);
        });
    }

    pub async fn start_connection(
        mut rx: Receiver<ConnectionThreadCommand>,
        addr: SocketAddr,
        username: String,
        mut app: AppHandle,
    ) -> io::Result<ExitReason> {
        println!("Starting connection...");
        let mut connection = Connection::connect(addr, username).await?;
        println!("Successfully connected!");
        println!("Token: {}", connection.get_token());

        let mut rx_enabled = true;

        println!("Starting server packet loop...");

        let lres = loop {
            let result: io::Result<()> = select! {
                biased;

                pres = connection.receive_packet() => {
                    println!("Received packet from server!");
                    let packet = pres?;
                    println!("Valid packet, started handling...");

                    match connection.handle_packet(packet, &mut app).await? {
                        PacketOutcome::Continue => Ok(()),
                        PacketOutcome::Exit => break ConnectionOutcome::Exit,
                        PacketOutcome::ExitWithoutCleanup => break ConnectionOutcome::ExitWithoutCleanup,
                    }
                }

                cres = rx.recv(), if rx_enabled => {
                    if let Some(command) = cres {
                        println!("Connection received external command.");
                        match command {
                            ConnectionThreadCommand::Packet(spnt) => {
                                println!("Sending packet to server.");
                                connection.send_packet(
                                    Packet::Serverbound(
                                        PacketH::from_spnt_to_sp(spnt, connection.get_token().to_string())
                                    )
                                ).await?;
                                println!("Sent packet to server!");
                            },
                            ConnectionThreadCommand::Disconnect => {
                                println!("Attempting disconnect.");
                                break ConnectionOutcome::Exit;
                            },
                        }
                    } else { rx_enabled = false; }

                    Ok(())
                }
            };

            if let Err(e) = result {
                println!("Breaking due to error from handling.");
                break ConnectionOutcome::Err(e);
            }

            println!("Command or packet handling was successful!");
        };

        println!("Closing connection because of: {:?}", lres);

        rx.close();

        match lres {
            ConnectionOutcome::Exit => {
                connection
                    .send_packet(PacketH::disconnect(connection.get_token().to_string()))
                    .await?;
            }
            ConnectionOutcome::ExitWithoutCleanup => {}
            ConnectionOutcome::Err(e) => {
                connection
                    .send_packet(PacketH::disconnect(connection.get_token().to_string()))
                    .await?;
                return Err(e);
            }
        }

        // allow for exit's to define a exitreason
        Ok(ExitReason::Disconnected)
    }
}
