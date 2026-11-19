use std::net::SocketAddr;
use tokio::net::TcpStream;

#[derive(Debug)]
pub struct TcpConnection {
    stream: TcpStream,
    peer: SocketAddr,
}

impl TcpConnection {
    pub(crate) fn new(stream: TcpStream, peer: SocketAddr) -> Self {
        Self { stream, peer }
    }

    pub fn peer_addr(&self) -> SocketAddr {
        self.peer
    }

    pub fn stream(&self) -> &TcpStream {
        &self.stream
    }

    pub fn into_stream(self) -> TcpStream {
        self.stream
    }
}
