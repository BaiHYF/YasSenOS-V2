#![no_std]
#![no_main]

use lib::*;

extern crate lib;

fn main() -> isize {

    println!("hello, this is a brk test!");

    let heap_start = sys_brk(None).unwrap();
    println!("Heap start: {:#x}", heap_start);
    let heap_end = heap_start + 0x1000;
    println!("Heap end: {:#x}", heap_end);

    let ret = sys_brk(Some(heap_end)).expect("Failed to allocate heap");

    assert!(ret == heap_end, "Failed to allocate heap");

    0
}

entry!(main);