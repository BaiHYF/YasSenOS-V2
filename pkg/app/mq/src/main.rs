#![no_std]
#![no_main]

extern crate alloc;
use lib::{collections::VecDeque, *};

extern crate lib;

#[derive(Debug)]
struct Message {
    pid: u16,
    val: usize,
}

const SIZE: u32 = 8;
static mut MQ: VecDeque<Message> = VecDeque::new();
static MUTEX: Semaphore = Semaphore::new(1);
static EMPTY: Semaphore = Semaphore::new(2);
static FULL: Semaphore = Semaphore::new(3);

impl Message {
    fn new(pid: u16, val: usize) -> Self {
        Message { pid, val }
    }
}

fn produce(pid: u16) {
    for i in 0..10 {
        let msg = Message::new(pid, i as usize);
        EMPTY.wait();
        MUTEX.wait();
        println!("producer pid: {}, send: {}, now queue length: {}", msg.pid, msg.val, unsafe { MQ.len() });
        unsafe {
            MQ.push_back(msg);
        }
        MUTEX.signal();
        FULL.signal();
    }
}

fn consume(pid: u16) {
    for _ in 0..10 {
        FULL.wait();
        MUTEX.wait();
        let msg = unsafe { MQ.pop_front().unwrap() };
        println!("consumer pid: {}, recv from {}: {}, now queue length: {}", pid, msg.pid, msg.val, unsafe { MQ.len() });
        MUTEX.signal();
        EMPTY.signal();
        //println!("pid: {}, val: {}", msg.pid, msg.val);
    }
}

fn main() -> isize {
    
    // 创建16个pid
    let mut pids = [0u16; 16];

    MUTEX.init(1);
    EMPTY.init(SIZE as usize);
    FULL.init(0);

    for i in 0..16 {
        let pid = sys_fork();
        if pid == 0 { // 子进程
            let pid = sys_get_pid();
            if i % 2 == 0 {
                produce(pid);
            } else {
                consume(pid);
            }
            sys_exit(0);
        } else { // 父进程
            pids[i as usize] = pid;
        }
    }

    sys_stat();

    for i in 0..16 {
        let pid = pids[i as usize];
        sys_wait_pid(pid);
    }

    println!("Message Queue Test Passed! is empty ? : {:?}", unsafe { MQ.is_empty() });

    0
}

entry!(main);