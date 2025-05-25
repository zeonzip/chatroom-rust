use std::io::{self, ErrorKind};

pub trait IoResult {
    fn to_io(self) -> io::Result<()>;
}

impl IoResult for tauri::Result<()> {
    fn to_io(self) -> io::Result<()> {
        self.map_err(|e| {
            io::Error::new(ErrorKind::NotConnected, "Coudln't connect to the frontend.")
        })
    }
}
