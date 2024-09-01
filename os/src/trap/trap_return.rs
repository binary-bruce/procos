use crate::{
    config::{TRAMPOLINE, TRAP_CONTEXT},
    task::current_user_token,
};
use core::arch::asm;
use riscv::register::{mtvec::TrapMode, stvec};

extern "C" {
    fn __alltraps();
    fn __restore();
}

#[no_mangle]
/// set the new addr of __restore asm function in TRAMPOLINE page,
/// set the reg a0 = trap_cx_ptr, reg a1 = phy addr of usr page table,
/// finally, jump to new addr of __restore asm function
pub fn trap_return() -> ! {
    set_user_trap_entry();

    let trap_cx_ptr = TRAP_CONTEXT;
    let user_satp = current_user_token();

    let restore_va = __restore as usize - __alltraps as usize + TRAMPOLINE;
    unsafe {
        asm!(
            "fence.i",
            "jr {restore_va}",
            restore_va = in(reg) restore_va,
            in("a0") trap_cx_ptr,
            in("a1") user_satp,
            options(noreturn)
        );
    }
}

fn set_user_trap_entry() {
    unsafe {
        stvec::write(TRAMPOLINE as usize, TrapMode::Direct);
    }
}
