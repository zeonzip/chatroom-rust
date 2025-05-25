use std::io;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use crate::server::ChatroomServer;

pub struct ChatroomServerWrapper(Arc<Mutex<ChatroomServer>>, TcpListener);

impl ChatroomServerWrapper {
    pub fn new(listener: TcpListener) -> ChatroomServerWrapper {
        ChatroomServerWrapper(Arc::new(Mutex::new(ChatroomServer::new())), listener)
    }

    pub async fn run(self) -> io::Result<()> {
        ChatroomServer::run(self.0, self.1).await
    }
}