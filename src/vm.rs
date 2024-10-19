use core::panic;
use std::u16;

pub use super::mem::*;
pub use super::regs::*;
pub use super::trap::*;

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

impl Trap {
    fn trap_puts(vm: &mut VM) {
        let mut addr = vm.regs.r0 as usize;
        let mut mem = vm.memory.get_mem(addr);
        while mem != 0 {
            print!("{}", (mem as u8) as char);
            addr += 1;
            mem = vm.memory.get_mem(addr);
        }

        // Flush the output
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
    }

    pub fn trap_out(vm: &mut VM) {
        let c = (vm.regs.r0 as u8) as char;
        print!("{}", c);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
    }

    pub fn trap_in(vm: &mut VM) {
        print!("Enter a character: ");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let c = input.chars().next().unwrap();

        print!("{}", c);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();

        vm.regs.r0 = c as u16;
        vm.update_flags(0x0);
    }

    pub fn trap_putsp(vm: &mut VM) {
        let mut addr = vm.regs.r0 as usize;
        let mut mem = vm.memory.get_mem(addr);

        while mem != 0 {
            let char1 = (mem & 0xFF) as u8 as char;
            let char2 = ((mem >> 8) & 0xFF) as u8 as char;

            print!("{}", char1);
            if char2 != '\0' {
                print!("{}", char2);
            }

            addr += 1;
            mem = vm.memory.get_mem(addr);
        }

        std::io::Write::flush(&mut std::io::stdout()).unwrap();
    }

    pub fn halt() {
        panic!("Normal Halt is not yet implemented due to complexity of the project!");
    }
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

    fn update_flags(&mut self, reg_num: u8) {
        let reg_val = self.regs.get_by_num(reg_num);
        if reg_val == 0 {
            self.regs.r_cond = Flags::FlZro as u16;
        } else if (reg_val >> 15) == 1 {
            self.regs.r_cond = Flags::FlNeg as u16;
        } else {
            self.regs.r_cond = Flags::FlPos as u16;
        }
    }

    fn op_add(&mut self, instr: u16) {
        /* Destination Register (DR) */
        let r0 = ((instr >> 9) & 0x7) as u8;
        /* First operand (SR1) */
        let r1 = ((instr >> 6) & 0x7) as u8;
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

    fn op_ldi(&mut self, instr: u16) {
        /* destination register (DR) */
        let r0 = ((instr >> 9) & 0x7) as u8;
        /* PCOffset9 */
        let pc_offset: u16 = self.sign_extend(instr & 0x1F, 9);

        let ptr: u16 = self
            .memory
            .get_mem((self.regs.r_progcount + pc_offset) as usize);

        self.regs.set_by_num(r0, self.memory.get_mem(ptr as usize));
        self.update_flags(r0);
    }

    fn op_and(&mut self, instr: u16) {
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
        self.update_flags(r0);
    }

    fn op_not(&mut self, instr: u16) {
        let r0 = ((instr >> 9) & 0x7) as u8;
        let r1 = ((instr >> 6) & 0x7) as u8;

        self.regs.set_by_num(r0, !self.regs.get_by_num(r1));
        self.update_flags(r0);
    }

    fn op_br(&mut self, instr: u16) {
        let pc_offset = self.sign_extend(instr & 0x1FF, 9);
        let cond_flag = (instr >> 9) & 0x7;
        if cond_flag & self.regs.r_cond != 0 {
            self.regs.r_progcount += pc_offset;
        }
    }

    fn op_jmp(&mut self, instr: u16) {
        /* Also handles RET since RET happens whenever R1 is 7 */
        let r1 = ((instr >> 6) & 0x7) as u8;
        self.regs.r_progcount = self.regs.get_by_num(r1);
    }

    fn op_jsr(&mut self, instr: u16) {
        let long_flag = (instr >> 11) & 1;
        self.regs.r7 = self.regs.r_progcount;
        if long_flag != 0 {
            let long_pc_offset = self.sign_extend(instr & 0x7FF, 11);
            self.regs.r_progcount += long_pc_offset;
        } else {
            let r1 = ((instr >> 6) & 0x7) as u8;
            self.regs.r_progcount = self.regs.get_by_num(r1);
        }
    }

    fn op_ld(&mut self, instr: u16) {
        let r0 = ((instr >> 9) & 0x7) as u8;
        let pc_offset = self.sign_extend(instr & 0x1FF, 9);
        self.regs.set_by_num(
            r0,
            self.memory
                .get_mem((self.regs.r_progcount + pc_offset) as usize),
        );
        self.update_flags(r0);
    }

    fn op_ldr(&mut self, instr: u16) {
        let r0 = ((instr >> 9) & 0x7) as u8;
        let r1 = ((instr >> 6) & 0x7) as u8;
        let offset = self.sign_extend(instr & 0x3F, 6);
        self.regs.set_by_num(
            r0,
            self.memory
                .get_mem((self.regs.get_by_num(r1) + offset) as usize),
        );
        self.update_flags(r0);
    }

    fn op_lea(&mut self, instr: u16) {
        let r0 = ((instr >> 9) & 0x7) as u8;
        let pc_offset = self.sign_extend(instr & 0x1FF, 9);
        self.regs.set_by_num(r0, self.regs.r_progcount + pc_offset);
        self.update_flags(r0);
    }

    fn op_st(&mut self, instr: u16) {
        let r0 = ((instr >> 9) & 0x7) as u8;
        let pc_offset = self.sign_extend(instr & 0x1FF, 9);
        self.memory.set_mem(
            (self.regs.r_progcount + pc_offset) as usize,
            self.regs.get_by_num(r0),
        );
    }

    fn op_sti(&mut self, instr: u16) {
        let r0 = ((instr >> 9) & 0x7) as u8;
        let pc_offset = self.sign_extend(instr & 0x1FF, 9);
        let mem_addr = self
            .memory
            .get_mem((self.regs.r_progcount + pc_offset) as usize) as usize;
        self.memory.set_mem(mem_addr, self.regs.get_by_num(r0));
    }

    fn op_str(&mut self, instr: u16) {
        let r0 = ((instr >> 9) & 0x7) as u8;
        let r1 = ((instr >> 6) & 0x7) as u8;
        let offset = self.sign_extend(instr & 0x3F, 6);
        self.memory.set_mem(
            (self.regs.get_by_num(r1) + offset) as usize,
            self.regs.get_by_num(r1),
        );
    }

    fn op_trap(&mut self, instr: u16) {
        match Trap::get_by_num(instr & 0xFF) {
            Trap::TrapGetc => (),
            Trap::TrapHalt => (),
            Trap::TrapIn => (),
            Trap::TrapOut => (),
            Trap::TrapPuts => (),
            Trap::TrapPutsp => (),
        }
    }

    pub fn execute(&mut self, cmd: Command) {
        match cmd.opcode {
            Opcode::OpADD => self.op_add(cmd.value),
            Opcode::OpLDI => self.op_ldi(cmd.value),
            Opcode::OpAND => self.op_and(cmd.value),
            Opcode::OpNOT => self.op_not(cmd.value),
            Opcode::OpBR => self.op_br(cmd.value),
            Opcode::OpJMP => self.op_jmp(cmd.value),
            Opcode::OpJSR => self.op_jsr(cmd.value),
            Opcode::OpLD => self.op_ld(cmd.value),
            Opcode::OpLDR => self.op_ldr(cmd.value),
            Opcode::OpLEA => self.op_lea(cmd.value),
            Opcode::OpST => self.op_st(cmd.value),
            Opcode::OpSTI => self.op_sti(cmd.value),
            Opcode::OpSTR => self.op_str(cmd.value),
            _ => panic!("Usage of reserved Opcodes!"),
        }
    }
}
