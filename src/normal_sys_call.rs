use std::io;

#[cfg(target_family = "unix")]
#[link(name = "c")]
unsafe extern "C" {
    fn write(fd: u32, buf: *const u8, count: usize) -> i32;
}

#[cfg(target_family = "unix")]
pub fn normal_syscall(message: &str) -> io::Result<()> {
    let msg_ptr = message.as_ptr();
    let len = message.len();
    let res = unsafe { write(1, msg_ptr, len) };

    if res == -1 {
        return Err(io::Error::last_os_error());
    }

    Ok(())
}
