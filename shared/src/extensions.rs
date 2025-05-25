// contains code for extensions of external libraries structs

use async_trait::async_trait;
use std::io::{self, ErrorKind};
use std::net::SocketAddr;
use std::time::Duration;
use tokio::net::TcpStream as TokioTcpStream;
use tokio::time::timeout;

#[async_trait]
pub trait ConnectTimeoutAsync {
    async fn connect_timeout(addr: &SocketAddr, dur: Duration) -> io::Result<TokioTcpStream>;
}

#[async_trait]
impl ConnectTimeoutAsync for TokioTcpStream {
    async fn connect_timeout(addr: &SocketAddr, dur: Duration) -> io::Result<TokioTcpStream> {
        match timeout(dur, TokioTcpStream::connect(addr)).await {
            Ok(Ok(stream)) => Ok(stream),
            Ok(Err(e)) => Err(e),
            Err(e) => Err(io::Error::new(ErrorKind::TimedOut, e)),
        }
    }
}
