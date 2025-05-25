use std::error;
use std::fmt::{Debug, Display, Formatter};
use std::fmt;
use std::io::ErrorKind;
use std::io::Error;

#[derive(Debug)]
pub enum ServerNetworkError {
    SentClientboundToServer,
    InvalidPacketFromClient,
}

impl Display for ServerNetworkError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ServerNetworkError::SentClientboundToServer => write!(f, "Client sent ClientBound Packet to server."),
            ServerNetworkError::InvalidPacketFromClient => write!(f, "Client sent a invalid packet to the server."),
        }
    }
}

impl error::Error for ServerNetworkError {}

impl From<ServerNetworkError> for std::io::Error {
    fn from(err: ServerNetworkError) -> Self {
        match err {
            ServerNetworkError::SentClientboundToServer => Error::new(ErrorKind::InvalidData, err),
            ServerNetworkError::InvalidPacketFromClient => Error::new(ErrorKind::InvalidData, err),
        }
    }
}