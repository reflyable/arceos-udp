use crate::io;
use axnet::SocketAddr;
use axnet::UdpSocket as _UdpSocket;

pub struct UdpSocket {
    socket: _UdpSocket,
}

impl UdpSocket {
    pub fn bind(addr: SocketAddr) -> io::Result<Self> {
        let mut socket = _UdpSocket::new();
        socket.bind(addr)?;
        Ok(Self { socket })
    }

    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.socket.local_addr()
    }

    pub fn recv_from(&self, buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        self.socket.recvfrom(buf)
    }

    pub fn send_to(&self, buf: &[u8], addr: SocketAddr) -> io::Result<usize> {
        self.socket.sendto(buf, addr)
    }
}
