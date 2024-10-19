pub use super::mem::{Mem, MEM_SIZE};
pub use super::regs::{Regs, REG_SIZE};

pub const PC_START: u16 = 0x3000;
pub struct VM {
    pub memory: Mem,
    pub regs: Regs,
}

pub enum Opcode {
    OpBR = 0x0,   /* branch */
    OpADD = 0x1,  /* add  */
    OpLD = 0x2,   /* load */
    OpST = 0x3,   /* store */
    OpJSR = 0x4,  /* jump register */
    OpAND = 0x5,  /* bitwise and */
    OpLDR = 0x6,  /* load register */
    OpSTR = 0x7,  /* store register */
    OpRTI = 0x8,  /* unused */
    OpNOT = 0x9,  /* bitwise not */
    OpLDI = 0xA,  /* load indirect */
    OpSTI = 0xB,  /* store indirect */
    OpJMP = 0xC,  /* jump */
    OpRES = 0xD,  /* reserved (unused) */
    OpLEA = 0xE,  /* load effective address */
    OpTRAP = 0xF, /* execute trap */
}

pub enum Flags {
    FlPos = 0x1,
    FlZro = 0x2,
    FlNeg = 0x4,
}

pub struct Command {
    opcode: Opcode,
    value: u16,
}

impl Command {
    pub fn new(val: u16) -> Self {
        Command {
            opcode: match val >> 12 {
                0x0 => Opcode::OpBR,
                0x1 => Opcode::OpADD,
                0x2 => Opcode::OpLD,
                0x3 => Opcode::OpST,
                0x4 => Opcode::OpJSR,
                0x5 => Opcode::OpAND,
                0x6 => Opcode::OpLDR,
                0x7 => Opcode::OpSTR,
                0x8 => Opcode::OpRTI,
                0x9 => Opcode::OpNOT,
                0xA => Opcode::OpLDI,
                0xB => Opcode::OpSTI,
                0xC => Opcode::OpJMP,
                0xD => Opcode::OpRES,
                0xE => Opcode::OpLEA,
                0xF => Opcode::OpTRAP,
                _ => Opcode::OpRES,
            },
            value: val,
        }
    }
}

impl VM {
    pub fn new() -> Self {
        VM {
            memory: Mem::new(),
            regs: Regs::new(),
        }
    }
}
