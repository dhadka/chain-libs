use std::net::SocketAddr;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Peer {
    addr: SocketAddr,
}

pub type Peers = Box<[Peer]>;

impl Peer {
    #[inline]
    pub fn addr(&self) -> SocketAddr {
        self.addr
    }
}

impl From<SocketAddr> for Peer {
    #[inline]
    fn from(addr: SocketAddr) -> Self {
        Peer { addr }
    }
}
