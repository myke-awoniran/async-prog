use std::arch::asm;
use std::io::{Error, ErrorKind};

fn main() -> Result<(), Error> {
    let t = 100;
    let t_ptr: *const usize = &t;
    let x = dereference(t_ptr);
    println!("{}", x);
    Ok(())
}

#[inline(always)]
fn dereference(ptr: *const usize) -> usize {
    let mut res: usize;
    unsafe { asm!("mov {0}, [{1}]", out(reg) res, in(reg) ptr) };
    res
}
