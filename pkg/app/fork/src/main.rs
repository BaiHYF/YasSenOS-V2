#![no_std]
#![no_main]

extern crate alloc;
use lib::*;

extern crate lib;

static mut M: u64 = 0xdeadbeef;

fn main() -> isize {
    let mut c = 32;

    let pid = sys_fork();

    if pid == 0 {
        println!("I am the child process");

        assert_eq!(c, 32);

        unsafe {
            println!("child read value of M: {:#x}", M);
            M = 0x2333;
            println!("child changed the value of M: {:#x}", M);
        }

        c += 32;
    } else {
        println!("I am the parent process");

        sys_stat();

        assert_eq!(c, 32);

        println!("Waiting for child to exit...");

        let ret = sys_wait_pid(pid);

        println!("Child exited with status {}", ret);

        assert_eq!(ret, 64);

        unsafe {
            println!("parent read value of M: {:#x}", M);
            assert_eq!(M, 0x2333);
        }

        c += 1024;

        assert_eq!(c, 1056);
    }

    c
}

entry!(main);