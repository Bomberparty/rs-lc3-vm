use crate::{
    constants::PC_START,
    enums::{Flags, Opcode, Trap},
    mem::Mem,
    regs::Regs,
};

pub struct VM {
    pub memory: Mem,
    pub regs: Regs,
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
            self.regs.set_by_num(8, Flags::FlZro as u16);
        } else if (reg_val >> 15) == 1 {
            self.regs.set_by_num(8, Flags::FlNeg as u16);
        } else {
            self.regs.set_by_num(8, Flags::FlPos as u16);
        }
    }

    fn op_add(&mut self, instr: u16) {
        let r0 = ((instr >> 9) & 0x7) as u8;
        let r1 = ((instr >> 6) & 0x7) as u8;
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
        let r0 = ((instr >> 9) & 0x7) as u8;
        let pc_offset = self.sign_extend(instr & 0x1FF, 9);
        let ptr = self.memory.get_mem((self.regs.get_by_num(9) + pc_offset) as usize);
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
        if cond_flag & self.regs.get_by_num(8) != 0 {
            self.regs.set_by_num(9, self.regs.get_by_num(9) + pc_offset);
        }
    }

    fn op_jmp(&mut self, instr: u16) {
        let r1 = ((instr >> 6) & 0x7) as u8;
        self.regs.set_by_num(9, self.regs.get_by_num(r1));
    }

    fn op_jsr(&mut self, instr: u16) {
        let long_flag = (instr >> 11) & 1;
        self.regs.set_by_num(7, self.regs.get_by_num(9));
        if long_flag != 0 {
            let long_pc_offset = self.sign_extend(instr & 0x7FF, 11);
            self.regs.set_by_num(9, self.regs.get_by_num(9) + long_pc_offset);
        } else {
            let r1 = ((instr >> 6) & 0x7) as u8;
            self.regs.set_by_num(9, self.regs.get_by_num(r1));
        }
    }

    fn op_ld(&mut self, instr: u16) {
        let r0 = ((instr >> 9) & 0x7) as u8;
        let pc_offset = self.sign_extend(instr & 0x1FF, 9);
        self.regs.set_by_num(
            r0,
            self.memory.get_mem((self.regs.get_by_num(9) + pc_offset) as usize),
        );
        self.update_flags(r0);
    }

    fn op_ldr(&mut self, instr: u16) {
        let r0 = ((instr >> 9) & 0x7) as u8;
        let r1 = ((instr >> 6) & 0x7) as u8;
        let offset = self.sign_extend(instr & 0x3F, 6);
        self.regs.set_by_num(
            r0,
            self.memory.get_mem((self.regs.get_by_num(r1) + offset) as usize),
        );
        self.update_flags(r0);
    }

    fn op_lea(&mut self, instr: u16) {
        let r0 = ((instr >> 9) & 0x7) as u8;
        let pc_offset = self.sign_extend(instr & 0x1FF, 9);
        self.regs.set_by_num(r0, self.regs.get_by_num(9) + pc_offset);
        self.update_flags(r0);
    }

    fn op_st(&mut self, instr: u16) {
        let r0 = ((instr >> 9) & 0x7) as u8;
        let pc_offset = self.sign_extend(instr & 0x1FF, 9);
        self.memory.set_mem(
            (self.regs.get_by_num(9) + pc_offset) as usize,
            self.regs.get_by_num(r0),
        );
    }

    fn op_sti(&mut self, instr: u16) {
        let r0 = ((instr >> 9) & 0x7) as u8;
        let pc_offset = self.sign_extend(instr & 0x1FF, 9);
        let mem_addr = self
            .memory
            .get_mem((self.regs.get_by_num(9) + pc_offset) as usize) as usize;
        self.memory.set_mem(mem_addr, self.regs.get_by_num(r0));
    }

    fn op_str(&mut self, instr: u16) {
        let r0 = ((instr >> 9) & 0x7) as u8;
        let r1 = ((instr >> 6) & 0x7) as u8;
        let offset = self.sign_extend(instr & 0x3F, 6);
        self.memory.set_mem(
            (self.regs.get_by_num(r1) + offset) as usize,
            self.regs.get_by_num(r0),
        );
    }

    fn op_trap(&mut self, instr: u16) {
        match Trap::get_by_num(instr & 0xFF) {
            Trap::TrapGetc => self.trap_getc(),
            Trap::TrapHalt => self.trap_halt(),
            Trap::TrapIn => self.trap_in(),
            Trap::TrapOut => self.trap_out(),
            Trap::TrapPuts => self.trap_puts(),
            Trap::TrapPutsp => self.trap_putsp(),
        }
    }

    fn trap_getc(&mut self) {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let c = input.chars().next().unwrap();
        self.regs.set_by_num(0, c as u16);
        self.update_flags(0);
    }

    fn trap_out(&mut self) {
        let c = (self.regs.get_by_num(0) as u8) as char;
        print!("{}", c);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
    }

    fn trap_puts(&mut self) {
        let mut addr = self.regs.get_by_num(0) as usize;
        let mut mem = self.memory.get_mem(addr);
        while mem != 0 {
            print!("{}", (mem as u8) as char);
            addr += 1;
            mem = self.memory.get_mem(addr);
        }
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
    }

    fn trap_in(&mut self) {
        print!("Enter a character: ");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let c = input.chars().next().unwrap();

        print!("{}", c);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();

        self.regs.set_by_num(0, c as u16);
        self.update_flags(0);
    }

    fn trap_putsp(&mut self) {
        let mut addr = self.regs.get_by_num(0) as usize;
        let mut mem = self.memory.get_mem(addr);

        while mem != 0 {
            let char1 = (mem & 0xFF) as u8 as char;
            let char2 = ((mem >> 8) & 0xFF) as u8 as char;

            print!("{}", char1);
            if char2 != '\0' {
                print!("{}", char2);
            }

            addr += 1;
            mem = self.memory.get_mem(addr);
        }

        std::io::Write::flush(&mut std::io::stdout()).unwrap();
    }

    fn trap_halt(&self) {
        panic!("Normal Halt is not yet implemented due to complexity of the project!");
    }

    pub fn execute(&mut self, instr: u16) {
        match instr >> 12 {
            0x0 => self.op_br(instr),
            0x1 => self.op_add(instr),
            0x2 => self.op_ld(instr),
            0x3 => self.op_st(instr),
            0x4 => self.op_jsr(instr),
            0x5 => self.op_and(instr),
            0x6 => self.op_ldr(instr),
            0x7 => self.op_str(instr),
            0x8 => self.op_rti(instr),
            0x9 => self.op_not(instr),
            0xA => self.op_ldi(instr),
            0xB => self.op_sti(instr),
            0xC => self.op_jmp(instr),
            0xD => self.op_res(instr),
            0xE => self.op_lea(instr),
            0xF => self.op_trap(instr),
            _ => panic!("Invalid opcode"),
        }
    }

    fn op_rti(&self, _instr: u16) {
        panic!("RTI is not implemented");
    }

    fn op_res(&self, _instr: u16) {
        panic!("RES is not implemented");
    }

    pub fn load_image(&mut self, image_path: &str) -> std::io::Result<()> {
        let mut file = std::fs::File::open(image_path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        let mut address = 0;
        for chunk in buffer.chunks(2) {
            if chunk.len() == 2 {
                let value = ((chunk[0] as u16) << 8) | (chunk[1] as u16);
                self.memory.set_mem(address, value);
            } else {
                panic!("Invalid binary image format");
            }
            address += 1;
        }

        Ok(())
    }

    pub fn run(&mut self) {
        self.regs.set_by_num(9, PC_START);

        loop {
            let instr = self.memory.get_mem(self.regs.get_by_num(9) as usize);
            self.regs.set_by_num(9, self.regs.get_by_num(9) + 1);
            self.execute(instr);
        }
    }
}