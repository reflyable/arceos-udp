#![no_std]
#![no_main]

#[macro_use]
extern crate libax;

extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use libax::io::{self, prelude::*};
use libax::net::{SocketAddr, TcpStream, ToSocketAddrs};

const DEST_HOST: &str = "ident.me";
const DEST_IP: &str = "49.12.234.183";

fn get_addr() -> SocketAddr {
    let dest = if cfg!(feature = "dns") {
        print!("{} ", DEST_HOST);
        DEST_HOST
    } else {
        DEST_IP
    };
    let addr_iter = (dest, 80).to_socket_addrs().unwrap().collect::<Vec<_>>();
    println!("IP:{}\n", addr_iter[0].addr);
    addr_iter[0]
}

fn client() -> io::Result {
    let requset: String =
        "GET / HTTP/1.1\r\nHost: ".to_string() + DEST_HOST + "\r\nAccept: */*\r\n\r\n";
    let mut stream = TcpStream::connect(get_addr())?;
    stream.write(requset.as_bytes())?;
    let mut buf = [0; 2048];
    let n = stream.read(&mut buf)?;
    let response = core::str::from_utf8(&buf[..n]).unwrap();
    println!("{}", response); // longer response need to handle tcp package problems.
    Ok(())
}

#[no_mangle]
fn main() {
    println!("Hello, simple http client!");
    client().expect("test http client failed");
}
