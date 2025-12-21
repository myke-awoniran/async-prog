mod fibers_and_green_threads;
mod raw_sys_Calls;
mod threads;

use raw_sys_Calls::syscall;
use std::arch::asm;
use std::io::{Error, ErrorKind};
use threads::run_threads;

fn main() -> Result<(), Error> {
    let message = String::from("Hell yea, Raw syscall calling in!\n");
    syscall(message);
    // run_threads();
    let t = 100;
    // let t_ptr: *const usize = &t;
    // let x = dereference(t_ptr);
    // println!("{}", x);
    Ok(())
}

#[inline(always)]
fn dereference(ptr: *const usize) -> usize {
    let mut res: usize;
    unsafe { asm!("mov {0}, [{1}]", out(reg) res, in(reg) ptr) };
    res
}
