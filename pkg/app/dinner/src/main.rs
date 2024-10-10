#![no_std]
#![no_main]

extern crate alloc;

extern crate lib;
use lib::*;

static CHOPSTICK: [Semaphore; 5] = semaphore_array!(0, 1, 2, 3, 4);
static WAITER: Semaphore = Semaphore::new(5);

fn dinner(set: u16) {

    // WAITER.wait(); // 进入餐厅

    let left: usize = set as usize;
    let right: usize = ((set + 1) % 5) as usize;
    if set % 2 == 0 {
        CHOPSTICK[left].wait();
        CHOPSTICK[right].wait();

        println!("Philosopher {} is eating", set);

        CHOPSTICK[right].signal();
        CHOPSTICK[left].signal();
    } else {
        CHOPSTICK[right].wait();
        CHOPSTICK[left].wait();

        println!("Philosopher {} is eating", set);

        CHOPSTICK[left].signal();
        CHOPSTICK[right].signal();
    }

    // WAITER.signal(); // 离开餐厅

}

fn main() -> isize {
    let mut pids = [0u16; 5];
    for i in 0..CHOPSTICK.len() {
        let pid = sys_fork();
        if pid == 0 { // child
            dinner(i as u16);
            sys_exit(0);
        } else {
            pids[i] = pid;
        }
    }    

    sys_stat();

    for i in 0..5 {
        sys_wait_pid(pids[i]);
    }

    0

}

entry!(main);