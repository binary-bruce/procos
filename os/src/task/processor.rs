//!Implementation of [`Processor`] and Intersection of control flow

use super::{TaskContext, TaskControlBlock};
use alloc::sync::Arc;

/// Processor management structure
pub struct Processor {
    /// The task currently executing on the current processor
    pub(crate) current: Option<Arc<TaskControlBlock>>,

    /// The basic control flow of each core, helping to select and switch process
    pub(crate) idle_task_cx: TaskContext,
}

impl Processor {
    /// Create an empty Processor
    pub fn new() -> Self {
        Self {
            current: None,
            idle_task_cx: TaskContext::zero_init(),
        }
    }

    /// Get mutable reference to `idle_task_cx`
    pub(crate) fn get_idle_task_cx_ptr(&mut self) -> *mut TaskContext {
        &mut self.idle_task_cx as *mut _
    }

    /// Get current task in moving semanteme
    pub fn take_current(&mut self) -> Option<Arc<TaskControlBlock>> {
        self.current.take()
    }

    /// Get current task in cloning semanteme
    pub fn current(&self) -> Option<Arc<TaskControlBlock>> {
        self.current.as_ref().map(Arc::clone)
    }
}
