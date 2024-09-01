#[repr(C)]
/// task context structure containing some registers
pub struct TaskContext {
    /// return address ( e.g. __restore ) of __switch ASM function
    ra: usize,

    /// kernel stack pointer of app
    kernel_sp: usize,

    /// s0-11 register, callee saved
    s: [usize; 12],
}

impl TaskContext {
    /// init task context
    pub fn zero_init() -> Self {
        Self {
            ra: 0,
            kernel_sp: 0,
            s: [0; 12],
        }
    }

    pub fn init(ra: usize, sp: usize) -> Self {
        Self {
            ra,
            kernel_sp: sp,
            s: [0; 12],
        }
    }
}
