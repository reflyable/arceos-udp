#![no_std]
#![no_main]

#[macro_use]
extern crate libax;

extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use libax::io::{self, prelude::*};
use libax::net::{TcpStream, ToSocketAddrs};

const DEST_HOST: &str = "www.example.com";

fn client() -> io::Result {
    let requset: String =
        "GET / HTTP/1.1\r\nHost: ".to_string() + DEST_HOST + "\r\nAccept: */*\r\n\r\n";
    let addr_iter = (DEST_HOST, 80)
        .to_socket_addrs()
        .unwrap()
        .collect::<Vec<_>>();
    println!("{} IP:{}\n", DEST_HOST, addr_iter[0].addr);
    let mut stream = TcpStream::connect(addr_iter[0])?;
    stream.write(requset.as_bytes())?;
    let mut buf = [0; 2048];
    let mut length;
    {
        let n = stream.read(&mut buf)?;
        debug!("{:?}", n);
        let response = core::str::from_utf8(&buf[..n]).unwrap();
        debug!("{:?}", response);
        let res: Vec<&str> = response.split("\r\n").collect();
        length = res
            .iter()
            .find(|s| s.to_ascii_lowercase().starts_with("content-length:"))
            .map(|s| {
                s["content-length:".len()..]
                    .trim()
                    .parse::<usize>()
                    .expect("not http")
            })
            .unwrap_or(0);
        debug!("{:?}", response);
        if length == 0 {
            println!("{}", response);
            return Ok(());
        }
        debug!("{:?}", length);
        let content = res[res.len() - 1];
        print!("{}", content);
        length -= content.as_bytes().len();
    }
    debug!("{:?}", length);
    while length != 0 {
        let n = stream.read(&mut buf)?;
        debug!("{:?}", n);
        debug!("{:?}", length);
        length -= n;
        let response = core::str::from_utf8(&buf[..n]).unwrap();
        print!("{}", response);
    }

    Ok(())
}

#[no_mangle]
fn main() {
    println!("Hello, simple http client!");
    client().expect("test http client failed");
}
