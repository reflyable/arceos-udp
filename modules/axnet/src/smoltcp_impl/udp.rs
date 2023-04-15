use axerrno::{ax_err, ax_err_type, AxError, AxResult};

use smoltcp::iface::SocketHandle;
use smoltcp::socket::udp::{self, BindError, SendError};

use super::{SocketSetWrapper, SOCKET_SET};
use crate::SocketAddr;

pub struct UdpSocket {
    handle: Option<SocketHandle>, // `None` if is listening
    local_addr: Option<SocketAddr>,
}

impl UdpSocket {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let socket = SocketSetWrapper::new_udp_socket();
        let handle = Some(SOCKET_SET.add(socket));
        Self {
            handle,
            local_addr: None,
        }
    }

    pub fn local_addr(&self) -> AxResult<SocketAddr> {
        self.local_addr.ok_or(AxError::NotConnected)
    }

    pub fn bind(&mut self, addr: SocketAddr) -> AxResult {
        debug!("{:?}", addr);
        let handle = self
            .handle
            .ok_or_else(|| ax_err_type!(InvalidInput, "socket bind() failed"))?;
        if self.local_addr.is_some() {
            return ax_err!(InvalidInput, "socket bind() failed: already bound");
        }
        debug!("{:?}", addr);
        SOCKET_SET.with_socket_mut::<udp::Socket, _, _>(handle, |socket| {
            socket.bind(addr).or_else(|e| match e {
                BindError::InvalidState => {
                    ax_err!(AlreadyExists, "socket bind() failed")
                }
                BindError::Unaddressable => {
                    ax_err!(InvalidInput, "socket bind() failed")
                }
            })?;
            Ok(socket.endpoint())
        })?;
        self.local_addr = Some(addr);
        Ok(())
    }

    pub fn sendto(&self, buf: &[u8], addr: SocketAddr) -> AxResult<usize> {
        let handle = self
            .handle
            .ok_or_else(|| ax_err_type!(InvalidInput, "socket bind() failed"))?;
        loop {
            SOCKET_SET.poll_interfaces();
            match SOCKET_SET.with_socket_mut::<udp::Socket, _, _>(handle, |socket| {
                if !socket.is_open() {
                    // not connected
                    ax_err!(NotConnected, "socket send() failed")
                } else if socket.can_send() {
                    // TODO: size
                    socket.send_slice(buf, addr).map_err(|e| match e {
                        SendError::BufferFull => AxError::Again,
                        SendError::Unaddressable => {
                            ax_err_type!(ConnectionRefused, "socket send() failed")
                        }
                    })?;
                    Ok(buf.len())
                } else {
                    // tx buffer is full
                    Err(AxError::Again)
                }
            }) {
                Ok(n) => {
                    debug!("here");
                    SOCKET_SET.poll_interfaces();
                    return Ok(n);
                }
                Err(AxError::Again) => axtask::yield_now(),
                Err(e) => return Err(e),
            }
        }
    }

    pub fn recvfrom(&self, buf: &mut [u8]) -> AxResult<(usize, SocketAddr)> {
        let handle = self
            .handle
            .ok_or_else(|| ax_err_type!(InvalidInput, "socket recv() failed"))?;
        loop {
            SOCKET_SET.poll_interfaces();
            match SOCKET_SET.with_socket_mut::<udp::Socket, _, _>(handle, |socket| {
                if !socket.is_open() {
                    // not connected
                    ax_err!(NotConnected, "socket recv() failed")
                } else if socket.can_recv() {
                    // data available
                    // TODO: use socket.recv(|buf| {...})
                    match socket.recv_slice(buf) {
                        Ok(x) => Ok(x),
                        Err(_) => Err(AxError::Again),
                    }
                } else {
                    // no more data
                    Err(AxError::Again)
                }
            }) {
                Ok(x) => {
                    SOCKET_SET.poll_interfaces();
                    return Ok(x);
                }
                Err(AxError::Again) => axtask::yield_now(),
                Err(e) => return Err(e),
            }
        }
    }

    pub fn shutdown(&self) -> AxResult {
        SOCKET_SET.poll_interfaces();
        if let Some(handle) = self.handle {
            // stream
            SOCKET_SET.with_socket_mut::<udp::Socket, _, _>(handle, |socket| {
                debug!("socket {}: shutting down", handle);
                socket.close();
            });
        } else {
            return ax_err!(InvalidInput, "socket shutdown() failed");
        }
        Ok(())
    }

    pub fn peekfrom(&self, buf: &mut [u8]) -> AxResult<(usize, SocketAddr)> {
        let handle = self
            .handle
            .ok_or_else(|| ax_err_type!(InvalidInput, "socket recv() failed"))?;
        loop {
            SOCKET_SET.poll_interfaces();
            match SOCKET_SET.with_socket_mut::<udp::Socket, _, _>(handle, |socket| {
                if !socket.is_open() {
                    // not connected
                    ax_err!(NotConnected, "socket recv() failed")
                } else if socket.can_recv() {
                    // data available
                    // TODO: use socket.recv(|buf| {...})
                    match socket.peek_slice(buf) {
                        Ok(x) => Ok((x.0, *x.1)),
                        Err(_) => Err(AxError::Again),
                    }
                } else {
                    // no more data
                    Err(AxError::Again)
                }
            }) {
                Ok(x) => {
                    SOCKET_SET.poll_interfaces();
                    return Ok(x);
                }
                Err(AxError::Again) => axtask::yield_now(),
                Err(e) => return Err(e),
            }
        }
    }
}

impl Drop for UdpSocket {
    fn drop(&mut self) {
        self.shutdown().ok();
        if let Some(handle) = self.handle {
            SOCKET_SET.remove(handle);
        }
    }
}
