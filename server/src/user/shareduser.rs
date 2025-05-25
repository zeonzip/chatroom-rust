// code for the shared user visible to all threads in the server.
// shared users should be able to take commands from other threads through a MPSC channel, and then the task channel for the private user should handle the commands.
use tokio::io;

use super::UserId;
use crate::consts::CHANNEL_BFR_SIZE;
use crate::utils::channel;
use crate::utils::channel::DuplexPeer;
use shared::packet::Packet;

pub enum CrossUserCommand {
    Packet(Packet),
    Feedback(Feedback),
    Disconnect,
    Kick(String),
    N,
}

pub enum Feedback {
    PacketSendResult(io::Result<()>),
    InvalidCommandData,
}

pub struct SharedUser {
    pub username: String,
    pub id: UserId,
    pub dp: DuplexPeer<CrossUserCommand>,
}

impl SharedUser {
    pub fn new(username: String, id: UserId) -> (DuplexPeer<CrossUserCommand>, SharedUser) {
        let (mut dp1, mut dp2) = channel::channel(CHANNEL_BFR_SIZE);

        (
            dp2,
            SharedUser {
                username,
                id,
                dp: dp1,
            },
        )
    }
}
