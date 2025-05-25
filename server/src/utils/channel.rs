use tokio::sync::mpsc;
use tokio::sync::mpsc::error::{SendError, TryRecvError};
use tokio::sync::mpsc::{Receiver, Sender};

pub struct DuplexPeer<T> {
    tx: Sender<T>,
    rx: Receiver<T>,
}

impl<T> DuplexPeer<T> {
    pub async fn recv(&mut self) -> Option<T> {
        self.rx.recv().await
    }

    pub fn try_recv(&mut self) -> Result<T, TryRecvError> {
        self.rx.try_recv()
    }

    pub async fn send(&mut self, val: T) -> Result<(), SendError<T>> {
        self.tx.send(val).await
    }
}

pub fn channel<T>(bfr_len: usize) -> (DuplexPeer<T>, DuplexPeer<T>) {
    let (tx1, mut rx1) = mpsc::channel(bfr_len);
    let (tx2, mut rx2) = mpsc::channel(bfr_len);

    (
        DuplexPeer { tx: tx1, rx: rx2 },
        DuplexPeer { tx: tx2, rx: rx1 },
    )
}
