use std::io;
use tokio::net::TcpListener;
use server::serverwrapper::ChatroomServerWrapper;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut listener = TcpListener::bind("127.0.0.1:6754").await?;
    let mut server = ChatroomServerWrapper::new(listener);

    server.run().await
}