use chrono::{naive::*, DateTime, Utc};
use syscall_def::Syscall;

#[inline(always)]
pub fn sys_write(fd: u8, buf: &[u8]) -> Option<usize> {
    let ret = syscall!(
        Syscall::Write,
        fd as u64,
        buf.as_ptr() as u64,
        buf.len() as u64
    ) as isize;
    if ret.is_negative() {
        None
    } else {
        Some(ret as usize)
    }
}

#[inline(always)]
pub fn sys_read(fd: u8, buf: &mut [u8]) -> Option<usize> {
    let ret = syscall!(
        Syscall::Read,
        fd as u64,
        buf.as_ptr() as u64,
        buf.len() as u64
    ) as isize;
    if ret.is_negative() {
        None
    } else {
        Some(ret as usize)
    }
}

#[inline(always)]
pub fn sys_allocate(layout: &core::alloc::Layout) -> *mut u8 {
    syscall!(Syscall::Allocate, layout as *const _) as *mut u8
}

#[inline(always)]
pub fn sys_list_app() {
    syscall!(Syscall::ListApp);
}

#[inline(always)]
pub fn sys_deallocate(ptr: *mut u8, layout: &core::alloc::Layout) -> usize {
    syscall!(Syscall::Deallocate, ptr, layout as *const _)
}

#[inline(always)]
pub fn sys_exit(code: usize) -> ! {
    syscall!(Syscall::Exit, code);
    unreachable!();
}

#[inline(always)]
pub fn sys_wait_pid(pid: u16) -> isize {
    syscall!(Syscall::WaitPid, pid as u64) as isize
}

#[inline(always)]
pub fn sys_time() -> DateTime<Utc> {
    let time = syscall!(Syscall::Time) as i64;
    const BILLION: i64 = 1_000_000_000;
    DateTime::from_timestamp(time / BILLION, (time % BILLION) as u32).unwrap_or_default()
}

#[inline(always)]
pub fn sys_stat() {
    syscall!(Syscall::Stat);
}

#[inline(always)]
pub fn sys_spawn(path: &str) -> u16 {
    syscall!(Syscall::Spawn, path.as_ptr() as u64, path.len() as u64) as u16
}

#[inline(always)]
pub fn sys_get_pid() -> u16 {
    syscall!(Syscall::GetPid) as u16
}

#[inline(always)]
pub fn sys_kill(pid: u16) {
    syscall!(Syscall::Kill, pid as u64);
}

#[inline(always)]
pub fn sys_fork() -> u16 {
    syscall!(Syscall::Fork) as u16
}

#[inline(always)] 
pub fn sys_new_sem(key: u32, val: usize) -> bool {
    syscall!(Syscall::Sem, 0, key as usize, val) == 0
}

#[inline(always)]
pub fn sys_del_sem(key: u32) -> bool {
    syscall!(Syscall::Sem, 1, key as usize, 0) == 0
}

#[inline(always)]
pub fn sys_signal_sem(key: u32) {
    syscall!(Syscall::Sem, 2, key as usize);
}

#[inline(always)]
pub fn sys_wait_sem(key: u32) {
    syscall!(Syscall::Sem, 3, key as usize);
}

#[inline(always)]
pub fn sys_brk(addr: Option<usize>) -> Option<usize> {
    const BRK_FAILED: usize = !0;
    match syscall!(Syscall::Brk, addr.unwrap_or(0)) {
        BRK_FAILED => None,
        ret => Some(ret),
    }
}