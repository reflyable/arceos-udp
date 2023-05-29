use super::ctypes;
use crate::io::{stdin, stdout};
use alloc::sync::Arc;
use alloc::vec::Vec;
use axerrno::{LinuxError, LinuxResult};
use axhal::time::current_time;
use core::{
    ffi::{c_int, c_void},
    time::Duration,
};
use flatten_objects::FlattenObjects;
use spin::RwLock;

pub const AX_FILE_LIMIT: usize = 1024;

pub trait FileLike: Send + Sync {
    fn read(&self, buf: &mut [u8]) -> LinuxResult<usize>;
    fn write(&self, buf: &[u8]) -> LinuxResult<usize>;
    fn stat(&self) -> LinuxResult<ctypes::stat>;
    fn into_any(self: Arc<Self>) -> Arc<dyn core::any::Any + Send + Sync>;
    fn is_ready(&self) -> LinuxResult<[bool; 3]>;
    fn set_status_flags(&self, arg: usize) -> LinuxResult;
}

lazy_static::lazy_static! {
    static ref FD_TABLE: RwLock<FlattenObjects<Arc<dyn FileLike>, AX_FILE_LIMIT>> = {
        let mut fd_table = FlattenObjects::new();
        fd_table.add_at(0, Arc::new(stdin()) as _).unwrap(); // stdin
        fd_table.add_at(1, Arc::new(stdout()) as _).unwrap(); // stdout
        fd_table.add_at(2, Arc::new(stdout()) as _).unwrap(); // stderr
        RwLock::new(fd_table)
    };
}

pub fn get_file_like(fd: c_int) -> LinuxResult<Arc<dyn FileLike>> {
    FD_TABLE
        .read()
        .get(fd as usize)
        .cloned()
        .ok_or(LinuxError::EBADF)
}

pub fn add_file_like(f: Arc<dyn FileLike>) -> LinuxResult<c_int> {
    Ok(FD_TABLE.write().add(f).ok_or(LinuxError::EMFILE)? as _)
}

pub fn close_file_like(fd: c_int) -> LinuxResult {
    let f = FD_TABLE
        .write()
        .remove(fd as usize)
        .ok_or(LinuxError::EBADF)?;
    drop(f);
    Ok(())
}

/// Close a file by `fd`.
#[no_mangle]
pub unsafe extern "C" fn ax_close(fd: c_int) -> c_int {
    debug!("ax_close <= {}", fd);
    if (0..2).contains(&fd) {
        return 0; // stdin, stdout, stderr
    }
    ax_call_body!(ax_close, close_file_like(fd).map(|_| 0))
}

/// Read data from the file indicated by `fd`.
///
/// Return the read size if success.
#[no_mangle]
pub unsafe extern "C" fn ax_read(fd: c_int, buf: *mut c_void, count: usize) -> ctypes::ssize_t {
    debug!("ax_read <= {} {:#x} {}", fd, buf as usize, count);
    ax_call_body!(ax_read, {
        if buf.is_null() {
            return Err(LinuxError::EFAULT);
        }
        let dst = unsafe { core::slice::from_raw_parts_mut(buf as *mut u8, count) };
        get_file_like(fd)?.read(dst)
    })
}

/// Write data to the file indicated by `fd`.
///
/// Return the written size if success.
#[no_mangle]
pub unsafe extern "C" fn ax_write(fd: c_int, buf: *const c_void, count: usize) -> ctypes::ssize_t {
    debug!("ax_write <= {} {:#x} {}", fd, buf as usize, count);
    ax_call_body!(ax_write, {
        if buf.is_null() {
            return Err(LinuxError::EFAULT);
        }
        let src = unsafe { core::slice::from_raw_parts(buf as *const u8, count) };
        get_file_like(fd)?.write(src)
    })
}

/// Get file metadata by `fd` and write into `buf`.
///
/// Return 0 if success.
#[no_mangle]
pub unsafe extern "C" fn ax_fstat(fd: c_int, buf: *mut ctypes::stat) -> ctypes::ssize_t {
    debug!("ax_fstat <= {} {:#x}", fd, buf as usize);
    ax_call_body!(ax_fstat, {
        if buf.is_null() {
            return Err(LinuxError::EFAULT);
        }
        unsafe { *buf = get_file_like(fd)?.stat()? };
        Ok(0)
    })
}

fn dup_fd(old_fd: c_int) -> LinuxResult<c_int> {
    let f = get_file_like(old_fd)?;
    let new_fd = add_file_like(f)?;
    Ok(new_fd)
}

/// Duplicate a file descriptor
#[no_mangle]
pub unsafe extern "C" fn ax_dup(old_fd: c_int) -> c_int {
    debug!("ax_dup <= {}", old_fd);
    ax_call_body!(ax_dup, dup_fd(old_fd))
}

/// `dup3()` is the same as `dup2()`, except that:
///
/// The caller can force the close-on-exec flag to be set for the new file descriptor by specifying `O_CLOEXEC` in flags.
///
/// If oldfd equals newfd, then `dup3()` fails with the error `EINVAL`.
#[no_mangle]
pub unsafe extern "C" fn ax_dup3(old_fd: c_int, new_fd: c_int, flags: c_int) -> c_int {
    debug!(
        "ax_dup3 <= old_fd: {}, new_fd: {}, flags: {}",
        old_fd, new_fd, flags
    );

    ax_call_body!(ax_dup3, {
        if old_fd == new_fd {
            return Err(LinuxError::EINVAL);
        }
        if new_fd as usize >= AX_FILE_LIMIT {
            return Err(LinuxError::EBADF);
        }

        let f = get_file_like(old_fd)?;
        FD_TABLE
            .write()
            .add_at(new_fd as usize, f)
            .ok_or(LinuxError::EMFILE)?;

        if flags as u32 & ctypes::O_CLOEXEC != 0 {
            ax_fcntl(
                new_fd,
                ctypes::F_SETFD as c_int,
                ctypes::FD_CLOEXEC as usize,
            );
        }
        Ok(new_fd)
    })
}

/// Fcntl implementation
///
/// TODO: `SET/GET` command is ignored
#[no_mangle]
pub unsafe extern "C" fn ax_fcntl(fd: c_int, cmd: c_int, arg: usize) -> c_int {
    debug!("ax_fcntl <= fd: {} cmd: {} arg: {}", fd, cmd, arg);
    ax_call_body!(ax_fcntl, {
        match cmd as u32 {
            ctypes::F_DUPFD => dup_fd(fd),
            ctypes::F_DUPFD_CLOEXEC => {
                // TODO: Change fd flags
                dup_fd(fd)
            }
            ctypes::F_SETFL => {
                get_file_like(fd)?.set_status_flags(arg)?;
                Ok(0)
            }
            _ => {
                warn!("unsupported fcntl parameters: cmd {}", cmd);
                Ok(0)
            }
        }
    })
}

fn getbit(fds: *mut ctypes::fd_set, n: usize) -> bool {
    assert!(n < 1024);
    ((unsafe { *fds }.fds_bits[n / 64]) & (1 << (n % 64))) > 0
}

fn setbit(fds: *mut ctypes::fd_set, n: usize) {
    assert!(n < 1024);

    unsafe {
        // debug!("    setbit: {} {:?}", n, (*fds).fds_bits);
        (*fds).fds_bits[n / 64] |= 1 << (n % 64);
        // debug!("    setbit: {} => {:?}", n, (*fds).fds_bits);
    };
}

fn clrfds(fds: *mut ctypes::fd_set) {
    unsafe { *fds = ctypes::fd_set { fds_bits: [0; 16] } };
}

/// Monitor multiple file descriptors, waiting until one or more of the file descriptors become "ready" for some class of I/O operation
#[no_mangle]
pub unsafe extern "C" fn ax_select(
    n: c_int,
    rfds: *mut ctypes::fd_set,
    wfds: *mut ctypes::fd_set,
    efds: *mut ctypes::fd_set,
    tv: *mut ctypes::timeval,
) -> c_int {
    debug!(
        "ax_select <= {} {:#x} {:#x} {:#x}",
        n, rfds as usize, wfds as usize, efds as usize
    );
    let fds = [rfds, wfds, efds];
    let mut res = Vec::<(u16, [bool; 3])>::new();
    for i in 0..1024 {
        let mut rec = [false; 3];
        for j in 0..3 {
            rec[j] = (!fds[j].is_null()) && (getbit(fds[j], i));
        }
        if rec[0] || rec[1] || rec[2] {
            res.push((i as u16, rec));
        }
    }
    for i in fds {
        if !i.is_null() {
            clrfds(i);
        }
    }
    let time: Option<Duration> = if tv.is_null() {
        None
    } else {
        Some(Duration::from_micros(unsafe {
            ((*tv).tv_sec * 1000 * 1000) as u64 + (*tv).tv_usec as u64
        }))
    };
    debug!("    fds: {:?} time: {:?}", res, time);
    ax_call_body!(ax_select, {
        let start_time = current_time();
        loop {
            if let Some(dur) = time {
                if dur < current_time() - start_time {
                    return Ok(0);
                }
            }
            let mut res_num = 0;
            for (index, r) in &res {
                let ready = get_file_like((*index).into())?.is_ready()?;
                for i in 0..3 {
                    if r[i] && ready[i] {
                        setbit(fds[i], (*index).into());
                        res_num += 1;
                    }
                }
            }
            if res_num > 0 {
                for i in fds {
                    if !i.is_null() {
                        debug!("    result {:?} ", (*i).fds_bits);
                    }
                }

                return Ok(res_num);
            }
        }
    })
}
