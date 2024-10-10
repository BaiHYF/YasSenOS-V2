#![no_std]
#![no_main]

extern crate alloc;

extern crate lib;
use lib::*;

static PROCESS: [Semaphore; 3] = semaphore_array!(0, 1, 2);
static WAITER: Semaphore = Semaphore::new(3);

fn print(id: usize) {
    let mut idx = 0;

    while idx < 10 {
        match id {
            0 => {

                PROCESS[0].wait();
                WAITER.wait();

                print!(">");
                // sleep(100);

                WAITER.signal();
                PROCESS[1].signal();
                
                PROCESS[0].wait();
                WAITER.wait();

                print!(">");
                // sleep(100);

                WAITER.signal();
                PROCESS[2].signal();

                idx += 1;
            
            }

            1 => {

                PROCESS[1].wait();

                print!("<");
                // sleep(100);

                PROCESS[0].signal();

                idx += 1;

            }

            2 => {

                PROCESS[2].wait();
                WAITER.wait();

                print!("_");
                // sleep(100);

                WAITER.signal();
                PROCESS[0].signal();

                idx += 1;

            }

            _ => unreachable!()
        }
        idx += 1;
        // println!("set {}, idx {}", id, idx);
    }
}

fn main() -> isize {
    let mut pids = [0u16; 3];

    for i in 0..3 {
        PROCESS[i].init(0);
    }
    WAITER.init(1);

    for i in 0..PROCESS.len() {
        let pid = sys_fork();
        if pid == 0 { // child
            print(i);
            sys_exit(0);
        } else {
            pids[i] = pid;
        }
    }

    PROCESS[0].signal();

    for i in 0..3 {
        sys_wait_pid(pids[i]);
    }


    println!("\nfish test done!");
    0

}

entry!(main);