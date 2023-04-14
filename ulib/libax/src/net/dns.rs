use axnet::IpAddr;
extern crate alloc;
use crate::io;
use axnet::DnsSocket;
pub fn resolve_socket_addr(name: &str) -> io::Result<alloc::vec::Vec<IpAddr>> {
    let socket = DnsSocket::new();
    socket.query(name, axnet::DnsQueryType::A)
}
