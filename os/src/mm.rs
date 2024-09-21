//! Memory management implementation
//!
//! SV39 page-based virtual-memory architecture for RV64 systems, and
//! everything about memory management, like frame allocator, page table,
//! map area and memory set, is implemented here.
//!
//! Every task or process has a memory_set to control its virtual memory.

mod kernel_memory_set;
mod kernel_stack;
mod user_memory_set;

pub use kernel_memory_set::KERNEL_SPACE;
pub use kernel_stack::KernelStack;
pub use memory_set::{MapPermission, MemorySet};
pub use page_table::{translated_byte_buffer, translated_str, PageTableEntry};

use crate::board::MEMORY_END;
use crate::config::KERNEL_HEAP_SIZE;

/// heap space ([u8; KERNEL_HEAP_SIZE])
static mut HEAP_SPACE: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];

pub use user_memory_set::*;

/// initiate heap allocator, frame allocator and kernel space
pub fn init() {
    extern "C" {
        fn ekernel();
    }

    // init heap so that dynamic memory allocation(e.g. Vec) is activated
    heap_allocator::init_heap(unsafe { HEAP_SPACE.as_ptr() } as usize, KERNEL_HEAP_SIZE);

    // manage physical memory frames for page table and application data
    page_table::init_frame_allocator(ekernel as usize, MEMORY_END);

    // enable virtual memory
    KERNEL_SPACE.exclusive_access().activate();
}
