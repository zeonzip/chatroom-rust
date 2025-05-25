use std::error;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::io::Error;
use std::io::ErrorKind;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum ClientNetworkError {
    SentServerboundToClient,
    UnexpectedPacket,
    UnreachableServer,
    NotResponding,
    UnimplementedPacket,
    Other(String),
}

impl Display for ClientNetworkError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ClientNetworkError::SentServerboundToClient => {
                write!(f, "Client sent ClientBound Packet to server.")
            }
            ClientNetworkError::UnexpectedPacket => {
                write!(f, "Client sent a invalid packet to the server.")
            }
            ClientNetworkError::UnreachableServer => {
                write!(f, "Server is unreachable.")
            }
            ClientNetworkError::NotResponding => {
                write!(f, "Server isn't responding.")
            }
            ClientNetworkError::UnimplementedPacket => {
                write!(f, "Packet is unimplemented.")
            }
            ClientNetworkError::Other(d) => {
                write!(f, "{}", d)
            }
        }
    }
}

impl error::Error for ClientNetworkError {}

impl From<ClientNetworkError> for std::io::Error {
    fn from(err: ClientNetworkError) -> Self {
        match err {
            ClientNetworkError::SentServerboundToClient => Error::new(ErrorKind::InvalidData, err),
            ClientNetworkError::UnexpectedPacket => Error::new(ErrorKind::InvalidData, err),
            ClientNetworkError::UnreachableServer => Error::new(ErrorKind::HostUnreachable, err),
            ClientNetworkError::NotResponding => Error::new(ErrorKind::TimedOut, err),
            ClientNetworkError::UnimplementedPacket => Error::new(ErrorKind::Unsupported, err),
            ClientNetworkError::Other(_) => Error::new(ErrorKind::Other, err),
        }
    }
}

#[derive(Debug, Serialize)]
pub enum CommandStatusError {
    InvalidIp,
    NoTransmitter,
}

impl Display for CommandStatusError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            CommandStatusError::InvalidIp => {
                write!(f, "Invalid IP entered!")
            }
            CommandStatusError::NoTransmitter => {
                write!(f, "No transmitter available.")
            }
        }
    }
}

impl error::Error for CommandStatusError {}

impl From<CommandStatusError> for std::io::Error {
    fn from(err: CommandStatusError) -> Self {
        match err {
            CommandStatusError::InvalidIp => Error::new(ErrorKind::InvalidData, err),
            CommandStatusError::NoTransmitter => Error::new(ErrorKind::NotFound, err),
        }
    }
}
