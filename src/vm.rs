use super::mem::{Mem, MEM_SIZE};
use super::regs::{Regs, REG_SIZE};

pub struct VM {
    memory: Mem,
    regs: Regs,
}

impl VM {
    pub fn new() -> Self {
        VM {
            memory: Mem::new(),
            regs: Regs::new(),
        }
    }
}
