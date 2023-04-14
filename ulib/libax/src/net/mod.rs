//! Networking primitives for TCP/UDP communication.

mod dns;
mod socket_addr;
mod tcp;
mod udp;

use self::dns::resolve_socket_addr;

pub use self::socket_addr::ToSocketAddrs;
pub use self::tcp::{TcpListener, TcpStream};
pub use self::udp::UdpSocket;
pub use axnet::{IpAddr, Ipv4Addr, SocketAddr};
