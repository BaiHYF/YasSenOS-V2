#![no_std]
#![no_main]

use lib::*;

extern crate lib;

static LOCK: SpinLock = lib::SpinLock::new();
const THREAD_COUNT: usize = 8;
static mut COUNTER: isize = 0;
static mut COUNTER_SEM: isize = 0;

fn main() -> isize {
    let pid = sys_fork();

    if pid == 0 {
        test_spin_lock();
        sys_exit(0);
    } else {
        test_semaphore();
        sys_wait_pid(pid);
    }

    println!("COUNTER result: {}", unsafe { COUNTER });
    println!("COUNTER_SEM result: {}", unsafe { COUNTER_SEM });

    0
}

fn test_spin_lock() {
    const HOLD: usize = THREAD_COUNT / 2;
    let mut pids = [0u16; HOLD];
    
    for i in 0..HOLD {
        let pid = sys_fork();
        if pid == 0 {
            do_counter_inc();

            sys_exit(0);
        } else {
            pids[i] = pid; // only parent knows child's pid
        }
    }

    let cpid = sys_get_pid();
    println!("process #{} holds threads: {:?}", cpid, &pids);
    sys_stat();

    for i in 0..HOLD {
        println!("#{} waiting for #{}...", cpid, pids[i]);
        sys_wait_pid(pids[i]);
    }


}

fn test_semaphore() {
    const HOLD: usize = THREAD_COUNT / 2;
    let mut pids = [0u16; HOLD];
    let key = 0x1234;
    sys_new_sem(key, 1);

    for i in 0..HOLD {
        let pid = sys_fork();
        if pid == 0 {
            do_counter_inc_semaphore();

            sys_exit(0);
        } else {
            pids[i] = pid; // only parent knows child's pid
        }
    }

    let cpid = sys_get_pid();
    println!("process #{} holds threads: {:?}", cpid, &pids);
    sys_stat();

    for i in 0..HOLD {
        println!("#{} waiting for #{}...", cpid, pids[i]);
        sys_wait_pid(pids[i]);
    }

    sys_del_sem(key);
}

fn do_counter_inc_semaphore() {
    let key = 0x1234;
    

    for _ in 0..100 {
        sys_wait_sem(key);
        inc_counter_sem();
        sys_signal_sem(key);
    }

}

fn do_counter_inc() {
    for _ in 0..100 {
        // FIXME: protect the critical section  
        LOCK.acquire();      
        inc_counter();
        LOCK.release();
    }
}

/// Increment the counter
///
/// this function simulate a critical section by delay
/// DO NOT MODIFY THIS FUNCTION
fn inc_counter() {
    unsafe {
        delay();
        let mut val = COUNTER;
        delay();
        val += 1;
        delay();
        COUNTER = val;
    }
}

fn inc_counter_sem() {
    unsafe {
        delay();
        let mut val = COUNTER_SEM;
        delay();
        val += 1;
        delay();
        COUNTER_SEM = val;
    }
}

#[inline(never)]
#[no_mangle]
fn delay() {
    for _ in 0..0x100 {
        core::hint::spin_loop();
    }
}

entry!(main);
