//!Implementation of [`TaskManager`]
use super::TaskControlBlock;
use crate::sync::UPSafeCell;
use alloc::collections::binary_heap::BinaryHeap;
use alloc::sync::Arc;
use lazy_static::*;

pub struct StrideTask {
    stride: usize,
    tcb: Arc<TaskControlBlock>,
}

impl Eq for StrideTask {}

impl PartialEq for StrideTask {
    fn eq(&self, other: &Self) -> bool {
        self.stride == other.stride
    }
}

impl Ord for StrideTask {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        other.stride.cmp(&self.stride)
    }
}

impl PartialOrd for StrideTask {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

///A array of `TaskControlBlock` that is thread-safe
pub struct TaskManager {
    ready_queue: BinaryHeap<StrideTask>,
}

/// A simple FIFO scheduler.
impl TaskManager {
    ///Creat an empty TaskManager
    pub fn new() -> Self {
        Self {
            ready_queue: BinaryHeap::new(),
        }
    }
    /// Add process back to ready queue
    pub fn add(&mut self, task: Arc<TaskControlBlock>) {
        self.ready_queue.push(StrideTask {
            stride: 0,
            tcb: task
        });
    }
    /// Take a process out of the ready queue
    pub fn fetch(&mut self) -> Option<Arc<TaskControlBlock>> {
        self.ready_queue.pop().map(|st| st.tcb)
    }

    /// put back to ready queue with the new stride
    pub fn put_back(&mut self, task: Arc<TaskControlBlock>, new_stride: usize) {
        self.ready_queue.push(StrideTask {
            stride: new_stride,
            tcb: task,
        });
    }
}

lazy_static! {
    /// TASK_MANAGER instance through lazy_static!
    pub static ref TASK_MANAGER: UPSafeCell<TaskManager> =
        unsafe { UPSafeCell::new(TaskManager::new()) };
}

/// Add process to ready queue
pub fn add_task(task: Arc<TaskControlBlock>) {
    //trace!("kernel: TaskManager::add_task");
    TASK_MANAGER.exclusive_access().add(task);
}

/// Take a process out of the ready queue
pub fn fetch_task() -> Option<Arc<TaskControlBlock>> {
    //trace!("kernel: TaskManager::fetch_task");
    TASK_MANAGER.exclusive_access().fetch()
}

/// put back to ready queue with the new stride
pub fn put_back_task(task: Arc<TaskControlBlock>, new_stride: usize) {
    TASK_MANAGER.exclusive_access().put_back(task, new_stride);
}