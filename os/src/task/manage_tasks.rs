use crate::task::task_manager::TaskManager;

use super::task_control_block::TaskControlBlock;
use alloc::sync::Arc;
use lazy_static::*;
use up_safe_cell::UPSafeCell;

lazy_static! {
    pub static ref TASK_MANAGER: UPSafeCell<TaskManager> =
        unsafe { UPSafeCell::new(TaskManager::new()) };
}

/// Interface offered to add task
pub fn add_task(task: Arc<TaskControlBlock>) {
    TASK_MANAGER.exclusive_access().add(task);
}

/// Interface offered to pop the first task
pub fn fetch_task() -> Option<Arc<TaskControlBlock>> {
    TASK_MANAGER.exclusive_access().fetch()
}
