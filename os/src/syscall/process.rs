//! Process management syscalls
use core::mem::{size_of};

use crate::{mm::{PTEFlags, PageTable, PhysAddr, VirtAddr, translated_byte_buffer}, syscall::map_syscall_id_to_index, task::{change_program_brk, current_mmap, current_munmap, current_user_token, exit_current_and_run_next, get_syscall_count, suspend_current_and_run_next}, timer::get_time_ms};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    unsafe {
        core::slice::from_raw_parts(p as *const T as *const u8, size_of::<T>())
    }
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let time = get_time_ms();
    let time_val = TimeVal { sec: time / 1000, usec: (time % 1000) * 1000 };
    let bytes = any_as_u8_slice(&time_val);

    let token = current_user_token();
    let buffers = translated_byte_buffer(token, ts as *const u8, bytes.len());
    let mut offset = 0;
    for buf in buffers {
        let len = buf.len();
        buf.copy_from_slice(&bytes[offset..offset+len]);
        offset += len;
    }

    0
}


/// TODO: Finish sys_trace to pass testcases
/// HINT: You might reimplement it with virtual memory management.
pub fn sys_trace(trace_request: usize, id: usize, data: usize) -> isize {
    trace!("kernel: sys_trace");
    match trace_request {
        0 => {
            let token = current_user_token();
            let pt = PageTable::from_token(token);
            let va = VirtAddr::from(id);
            let vpn = va.floor();

            if let Some(pte) = pt.translate(vpn) {
                if pte.is_valid() && pte.flags().contains(PTEFlags::U) && pte.readable() {
                    let pa: PhysAddr = pte.ppn().into();
                    let addr = pa.0 + va.page_offset();
                    return unsafe { *(addr as *const u8) as isize }
                }
                return -1;
            }
            return -1;
        },
        1 => {
            let token = current_user_token();
            let pt = PageTable::from_token(token);
            let va = VirtAddr::from(id);
            let vpn = va.floor();

            if let Some(pte) = pt.translate(vpn) {
                if pte.is_valid() && pte.flags().contains(PTEFlags::U) && pte.writable() {
                    let pa = PhysAddr::from(pte.ppn());
                    let addr = pa.0 + va.page_offset();
                    unsafe {
                        *(addr as *mut u8) = data as u8;
                    }
                    return 0;
                }
                return -1;
            }
            return -1;
        }
        2 => {
            return get_syscall_count(map_syscall_id_to_index(id).unwrap()) as isize
        },
        _ => return -1
    }
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(start: usize, len: usize, port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    current_mmap(start, len, port)
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(start: usize, len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    current_munmap(start, len)
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
