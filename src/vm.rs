use core::panic;
use std::u16;

pub use super::mem::*;
pub use super::regs::*;

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
                _ => panic!("Incorrect Opcode inside of the command constructor"),
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

    fn sign_extend(&self, value: u16, bit_count: u16) -> u16 {
        if (value >> (bit_count - 1)) & 1 != 0 {
            value | (0xFFFF << bit_count)
        } else {
            value
        }
    }

    pub fn update_flags(&mut self, reg_num: u8) {
        let reg_val = self.regs.get_by_num(reg_num);
        if reg_val == 0 {
            self.regs.r_cond = Flags::FlZro as u16;
        } else if (reg_val >> 15) == 1 {
            self.regs.r_cond = Flags::FlNeg as u16;
        } else {
            self.regs.r_cond = Flags::FlPos as u16;
        }
    }

    pub fn op_add(&mut self, instr: u16) {
        /* Destination Register (DR) */
        let r0 = ((instr >> 9) & 0x7) as u8;
        /* First operand (SR1) */
        let r1 = ((instr >> 7) & 0x7) as u8;
        /* Immediate flag */
        let imm_flag = (instr >> 5) & 0x1 != 0;

        if imm_flag {
            self.regs.set_by_num(
                r0,
                self.regs.get_by_num(r1) + self.sign_extend(instr & 0x1F, 5),
            );
        } else {
            self.regs
                .set_by_num(r0, self.regs.get_by_num(r1) + (instr & 0x7));
        }

        self.update_flags(r0);
    }

    pub fn op_ldi(&mut self, instr: u16) {
        /* destination register (DR) */
        let r0: u16 = (instr >> 9) & 0x7;
        /* PCOffset9 */
        let pc_offset: u16 = self.sign_extend(instr & 0x1F, 9);

        let ptr: u16 = self
            .memory
            .get_mem((self.regs.r_progcount + pc_offset) as usize);

        self.regs
            .set_by_num(r0 as u8, self.memory.get_mem(ptr as usize));
    }

    pub fn op_and(&mut self, instr: u16) {
        let r0 = ((instr >> 9) & 0x7) as u8;
        let r1 = ((instr >> 6) & 0x7) as u8;
        let imm_flag = ((instr >> 5) & 0x1) != 0;

        if imm_flag {
            self.regs.set_by_num(
                r0,
                self.regs.get_by_num(r1) & self.sign_extend(instr & 0x1F, 5),
            );
        } else {
            self.regs
                .set_by_num(r0, self.regs.get_by_num(r1) & (instr & 0x7));
        }
    }

    pub fn execute(&mut self, cmd: Command) {
        match cmd.opcode {
            Opcode::OpADD => self.op_add(cmd.value),
            Opcode::OpLDI => self.op_ldi(cmd.value),
            Opcode::OpAND => self.op_and(cmd.value),
            _ => panic!("Usage of reserved Opcodes!"),
        }
    }
}
