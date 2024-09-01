//! Implementation of [`MapArea`] and [`MemorySet`].

use crate::config::{MEMORY_END, MMIO, TRAMPOLINE};

use alloc::sync::Arc;
use lazy_static::*;
use memory_set::*;
use up_safe_cell::UPSafeCell;

extern "C" {
    fn stext();
    fn etext();
    fn srodata();
    fn erodata();
    fn sdata();
    fn edata();
    fn sbss_with_stack();
    fn ebss();
    fn ekernel();
    fn strampoline();
}

lazy_static! {
    /// a memory set instance through lazy_static! managing kernel space
    pub static ref KERNEL_SPACE: Arc<UPSafeCell<MemorySet>> =
        Arc::new(unsafe { UPSafeCell::new(new_kernel()) });
}

/// Without kernel stacks.
fn new_kernel() -> MemorySet {
    // map kernel sections
    println!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
    println!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
    println!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
    println!(
        ".bss [{:#x}, {:#x})",
        sbss_with_stack as usize, ebss as usize
    );
    println!("mapping .text section");
    let rx = MapPermission::R | MapPermission::X;
    let r = MapPermission::R;
    let rw = MapPermission::R | MapPermission::W;
    let mut memory_set_builder = MemorySetBuilder::new()
        .map_trampoline(TRAMPOLINE, strampoline as usize)
        .push_identical(stext as usize, etext as usize, rx)
        .push_identical(srodata as usize, erodata as usize, r)
        .push_identical(sdata as usize, edata as usize, rw)
        .push_identical(sbss_with_stack as usize, ebss as usize, rw)
        .push_identical(ekernel as usize, MEMORY_END, rw);

    println!("mapping memory-mapped registers");
    for pair in MMIO {
        memory_set_builder =
            memory_set_builder.push_identical((*pair).0, (*pair).0 + (*pair).1, rw);
    }

    memory_set_builder.build()
}
