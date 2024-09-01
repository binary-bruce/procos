//! Implementation of [`MapArea`] and [`MemorySet`].

use crate::config::{TRAMPOLINE, TRAP_CONTEXT, USER_STACK_SIZE};

use memory_set::MemorySet;

extern "C" {
    fn strampoline();
}

/// Include sections in elf and trampoline and TrapContext and user stack,
/// also returns user_sp and entry point.
pub fn from_elf(elf_data: &[u8]) -> (MemorySet, usize, usize) {
    MemorySet::from_elf(
        elf_data,
        TRAMPOLINE,
        strampoline as usize,
        TRAP_CONTEXT,
        USER_STACK_SIZE,
    )
}

pub fn from_existed_user(user_space: &MemorySet) -> MemorySet {
    MemorySet::from_existed_user(user_space, TRAMPOLINE, strampoline as usize)
}
