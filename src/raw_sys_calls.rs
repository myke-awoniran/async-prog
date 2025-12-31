// What does this file do?
// Telling rust to use the syscall instruction on macos aarch64(Newer mac chips to
// to print to the console using raw syscalls instead of std::io::stdout or println!
use ::std::arch::asm;

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
#[inline(never)]
pub fn syscall(message: &str) {
    let ptr = message.as_ptr();
    let len = message.len();
    unsafe {
        asm!(
        "mov x16, 4",
        "mov x0, 1",
        "svc 0",
        in("x1") ptr,
        in("x2") len,
        out("x16") _,
        out("x0") _,
        lateout("x1") _,
        lateout("x2") _
        );
    }
}
