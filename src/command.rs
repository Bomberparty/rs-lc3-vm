#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Register {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
}

#[derive(Debug)]
pub enum Command {
    ADDReg {
        dr: Register,
        sr1: Register,
        sr2: Register,
    },
    ADDImm {
        dr: Register,
        sr1: Register,
        imm5: u16,
    },
    ANDReg {
        dr: Register,
        sr1: Register,
        sr2: Register,
    },
    ANDImm {
        dr: Register,
        sr1: Register,
        imm5: u16,
    },
    BR {
        flag: u16,
        pc_offset9: i16,
    },
    JMP {
        base_r: Register,
    },
    JSR {
        pc_offset11: i16,
    },
    JSRR {
        base_r: Register,
    },
    LD {
        dr: Register,
        pc_offset9: i16,
    },
    LDI {
        dr: Register,
        pc_offset9: i16,
    },
    LDR {
        dr: Register,
        base_r: Register,
        offset6: i16,
    },
    LEA {
        dr: Register,
        pc_offset9: i16,
    },
    NOT {
        dr: Register,
        sr: Register,
    },
    RET,
    RTI,
    ST {
        r: Register,
        pc_offset9: i16,
    },
    STI {
        r: Register,
        pc_offset9: i16,
    },
    STR {
        sr: Register,
        base_r: Register,
        offset6: i16,
    },
    TRAP {
        trap_vec: TrapVector,
    },
    RES,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TrapVector {
    GETC,
    OUT,
    PUTS,
    IN,
    PUTSP,
    HALT,
}

impl Command {
    pub fn from_u16(instruction: u16) -> Self {
        let opcode = (instruction >> 12) & 0xF;
        match opcode {
            0x1 => {
                let dr = ((instruction >> 9) & 0x7).into();
                let sr1 = ((instruction >> 6) & 0x7).into();
                let imm_flag = (instruction & 0x20) != 0;
                if imm_flag {
                    let imm5 = (instruction & 0x1F) as u16;
                    Command::ADDImm { dr, sr1, imm5 }
                } else {
                    let sr2 = ((instruction & 0x7) as u16).into();
                    Command::ADDReg { dr, sr1, sr2 }
                }
            }
            0x5 => {
                let dr = ((instruction >> 9) & 0x7).into();
                let sr1 = ((instruction >> 6) & 0x7).into();
                let imm_flag = (instruction & 0x20) != 0;
                if imm_flag {
                    let imm5 = (instruction & 0x1F) as u16;
                    Command::ANDImm { dr, sr1, imm5 }
                } else {
                    let sr2 = ((instruction & 0x7) as u16).into();
                    Command::ANDReg { dr, sr1, sr2 }
                }
            }
            0x0 => {
                let flag = (instruction >> 9) & 0x7;
                let pc_offset9 = ((instruction & 0x1FF) as i16).wrapping_shl(7) >> 7;
                Command::BR { flag, pc_offset9 }
            }
            0xC => {
                let base_r = ((instruction >> 6) & 0x7).into();
                Command::JMP { base_r }
            }
            0x4 => {
                let jsr_flag = (instruction >> 11) & 0x1;
                if jsr_flag == 1 {
                    let pc_offset11 = ((instruction & 0x7FF) as i16).wrapping_shl(5) >> 5;
                    Command::JSR { pc_offset11 }
                } else {
                    let base_r = ((instruction >> 6) & 0x7).into();
                    Command::JSRR { base_r }
                }
            }
            0x2 => {
                let dr = ((instruction >> 9) & 0x7).into();
                let pc_offset9 = ((instruction & 0x1FF) as i16).wrapping_shl(7) >> 7;
                Command::LD { dr, pc_offset9 }
            }
            0xA => {
                let dr = ((instruction >> 9) & 0x7).into();
                let pc_offset9 = ((instruction & 0x1FF) as i16).wrapping_shl(7) >> 7;
                Command::LDI { dr, pc_offset9 }
            }
            0x6 => {
                let dr = ((instruction >> 9) & 0x7).into();
                let base_r = ((instruction >> 6) & 0x7).into();
                let offset6 = ((instruction & 0x3F) as i16).wrapping_shl(10) >> 10;
                Command::LDR {
                    dr,
                    base_r,
                    offset6,
                }
            }
            0xE => {
                let dr = ((instruction >> 9) & 0x7).into();
                let pc_offset9 = ((instruction & 0x1FF) as i16).wrapping_shl(7) >> 7;
                Command::LEA { dr, pc_offset9 }
            }
            0x9 => {
                let dr = ((instruction >> 9) & 0x7).into();
                let sr = ((instruction >> 6) & 0x7).into();
                Command::NOT { dr, sr }
            }
            0xC => Command::RET,
            0x8 => Command::RTI,
            0x3 => {
                let r = ((instruction >> 9) & 0x7).into();
                let pc_offset9 = ((instruction & 0x1FF) as i16).wrapping_shl(7) >> 7;
                Command::ST { r, pc_offset9 }
            }
            0xB => {
                let r = ((instruction >> 9) & 0x7).into();
                let pc_offset9 = ((instruction & 0x1FF) as i16).wrapping_shl(7) >> 7;
                Command::STI { r, pc_offset9 }
            }
            0x7 => {
                let sr = ((instruction >> 9) & 0x7).into();
                let base_r = ((instruction >> 6) & 0x7).into();
                let offset6 = ((instruction & 0x3F) as i16).wrapping_shl(10) >> 10;
                Command::STR {
                    sr,
                    base_r,
                    offset6,
                }
            }
            0xF => {
                let trap_vec8 = (instruction & 0xFF) as u8;
                let trap_vec = match trap_vec8 {
                    0x20 => TrapVector::GETC,
                    0x21 => TrapVector::OUT,
                    0x22 => TrapVector::PUTS,
                    0x23 => TrapVector::IN,
                    0x24 => TrapVector::PUTSP,
                    0x25 => TrapVector::HALT,
                    _ => panic!("Invalid trap vector"),
                };
                Command::TRAP { trap_vec }
            }
            _ => Command::RES,
        }
    }
}

impl From<u16> for Register {
    fn from(value: u16) -> Self {
        match value {
            0 => Register::R0,
            1 => Register::R1,
            2 => Register::R2,
            3 => Register::R3,
            4 => Register::R4,
            5 => Register::R5,
            6 => Register::R6,
            7 => Register::R7,
            _ => panic!("Invalid register value"),
        }
    }
}