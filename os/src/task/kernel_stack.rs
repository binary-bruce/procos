//!Implementation of [`PidAllocator`]
use crate::config::{KERNEL_STACK_SIZE, PAGE_SIZE, TRAMPOLINE};
use crate::mm::{MapPermission, KERNEL_SPACE};
use page_table::VirtAddr;
use pid::PidHandle;

pub struct KernelStack {
    pid: usize,
}

impl KernelStack {
    /// Create a kernelstack from pid
    pub fn new(pid_handle: &PidHandle) -> Self {
        let pid = pid_handle.0;
        let (kernel_stack_bottom, kernel_stack_top) = Self::kernel_stack_position(pid);
        KERNEL_SPACE.exclusive_access().insert_framed_area(
            kernel_stack_bottom.into(),
            kernel_stack_top.into(),
            MapPermission::R | MapPermission::W,
        );
        KernelStack { pid: pid_handle.0 }
    }

    #[allow(unused)]
    /// Push a value on top of kernelstack
    pub fn push_on_top<T>(&self, value: T) -> *mut T
    where
        T: Sized,
    {
        let kernel_stack_top = self.get_top();
        let ptr_mut = (kernel_stack_top - core::mem::size_of::<T>()) as *mut T;
        unsafe {
            *ptr_mut = value;
        }
        ptr_mut
    }

    /// Get the value on the top of kernelstack
    pub fn get_top(&self) -> usize {
        let (_, kernel_stack_top) = Self::kernel_stack_position(self.pid);
        kernel_stack_top
    }

    /// Return (bottom, top) of a kernel stack in kernel space.
    pub fn kernel_stack_position(app_id: usize) -> (usize, usize) {
        let top = TRAMPOLINE - app_id * (KERNEL_STACK_SIZE + PAGE_SIZE);
        let bottom = top - KERNEL_STACK_SIZE;
        (bottom, top)
    }
}

impl Drop for KernelStack {
    fn drop(&mut self) {
        let (kernel_stack_bottom, _) = KernelStack::kernel_stack_position(self.pid);
        let kernel_stack_bottom_va: VirtAddr = kernel_stack_bottom.into();
        KERNEL_SPACE
            .exclusive_access()
            .remove_area_with_start_vpn(kernel_stack_bottom_va.into());
    }
}
