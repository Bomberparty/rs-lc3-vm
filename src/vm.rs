use crate::command::{Register, TrapVector};

use super::command::Command;
use std::io::{self, Read};

pub struct VM {
    registers: [u16; 8],
    flag: u16,
    memory: [u16; 65536],
}

impl VM {
    pub fn new() -> Self {
        VM {
            registers: [0; 8],
            flag: 0,
            memory: [0; 65536],
        }
    }

    pub fn load_image(&mut self, buffer: &[u8]) -> io::Result<()> {
        if buffer.len() < 2 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "File too short"));
        }

        let start_address = self.extract_start_address(buffer);
        let mut index = 2;

        while index + 1 < buffer.len() {
            if start_address as usize + (index - 2) / 2 >= self.memory.len() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Start address out of bounds",
                ));
            }

            let instruction = ((buffer[index] as u16) << 8) | buffer[index + 1] as u16;
            self.memory[start_address as usize + (index - 2) / 2] = instruction;
            index += 2;
        }

        Ok(())
    }

    pub fn run(&mut self, buffer: &[u8]) -> io::Result<()> {
        let start_address = self.extract_start_address(buffer);
        let mut pc = start_address;

        loop {
            if pc as usize >= self.memory.len() {
                break;
            }

            let instruction = self.memory[pc as usize];
            let command = Command::from_u16(instruction);
            pc = (pc + 1) % u16::MAX;

            match command {
                Command::ADDImm { dr, sr1, imm5 } => {
                    self.registers[dr as usize] = self.registers[sr1 as usize] + imm5;
                    self.update_flags(self.registers[dr as usize]);
                }
                Command::ADDReg { dr, sr1, sr2 } => {
                    self.registers[dr as usize] =
                        self.registers[sr1 as usize] + self.registers[sr2 as usize];
                    self.update_flags(self.registers[dr as usize]);
                }
                Command::BR { flag, pc_offset9 } => {
                    if flag & self.flag != 0 {
                        pc = pc.wrapping_add_signed(pc_offset9).wrapping_add_signed(-1);
                    }
                }
                Command::JMP { base_r } => {
                    pc = self.memory[self.registers[base_r as usize] as usize];
                }
                Command::JSR { pc_offset11 } => {
                    self.registers[Register::R7 as usize] = pc;
                    pc = pc.wrapping_add_signed(pc_offset11);
                }
                Command::JSRR { base_r } => {
                    self.registers[Register::R7 as usize] = pc;
                    pc = self.registers[base_r as usize];
                }
                Command::LD { dr, pc_offset9 } => {
                    self.registers[dr as usize] =
                        self.memory[pc.wrapping_add_signed(pc_offset9) as usize];
                }
                Command::LDI { dr, pc_offset9 } => {
                    self.registers[dr as usize] = self.memory
                        [self.memory[pc.wrapping_add_signed(pc_offset9) as usize] as usize];
                }
                Command::LDR {
                    dr,
                    base_r,
                    offset6,
                } => {
                    self.registers[dr as usize] = self.memory
                        [(self.registers[base_r as usize].wrapping_add_signed(offset6)) as usize];
                }
                Command::LEA { dr, pc_offset9 } => {
                    self.registers[dr as usize] = pc.wrapping_add_signed(pc_offset9);
                }
                Command::NOT { dr, sr } => {
                    self.registers[dr as usize] = !self.registers[sr as usize];
                }
                Command::RET => {
                    pc = self.registers[Register::R7 as usize];
                }
                Command::ST { r, pc_offset9 } => {
                    self.memory[pc.wrapping_add_signed(pc_offset9) as usize] =
                        self.registers[r as usize];
                }
                Command::STI { r, pc_offset9 } => {
                    self.memory
                        [self.memory[pc.wrapping_add_signed(pc_offset9) as usize] as usize] =
                        self.registers[r as usize];
                }
                Command::STR {
                    sr,
                    base_r,
                    offset6,
                } => {
                    self.memory
                        [(self.registers[base_r as usize].wrapping_add_signed(offset6)) as usize] =
                        self.registers[sr as usize];
                }
                Command::TRAP { trap_vec } => match trap_vec {
                    TrapVector::GETC => {
                        let mut buffer = [0u8; 1];
                        io::stdin().read_exact(&mut buffer)?;
                        self.registers[Register::R0 as usize] = buffer[0] as u16;
                    }
                    TrapVector::HALT => {
                        println!("Halting the processor...");
                        return Ok(());
                    }
                    TrapVector::IN => {
                        print!("Enter the character: ");
                        let mut buffer = [0u8; 1];
                        io::stdin().read_exact(&mut buffer)?;
                        self.registers[Register::R0 as usize] = buffer[0] as u16;
                    }
                    TrapVector::OUT => {
                        print!("{}", self.registers[Register::R0 as usize] as u8 as char);
                    }
                    TrapVector::PUTS => {
                        let slice = &self.memory[self.registers[Register::R0 as usize] as usize..];
                        let string = self.u16_slice_to_string(slice);
                        println!("{}", string);
                    }
                    TrapVector::PUTSP => {
                        let slice = &self.memory[self.registers[Register::R0 as usize] as usize..];
                        let string = self.u16_slice_to_string(slice);
                        for byte in string.as_bytes() {
                            print!("{:X}", byte);
                        }
                        println!("");
                    }
                },
                _ => panic!("Wrong command used!"),
            }
        }

        Ok(())
    }

    fn extract_start_address(&self, buffer: &[u8]) -> u16 {
        ((buffer[0] as u16) << 8) | buffer[1] as u16
    }

    fn update_flags(&mut self, value: u16) {
        if value == 0 {
            self.flag = 0x2; // FlZro
        } else if value & 0x8000 != 0 {
            self.flag = 0x4; // FlNeg
        } else {
            self.flag = 0x1; // FlPos
        }
    }

    fn u16_slice_to_string(&self, slice: &[u16]) -> String {
        let mut result = String::new();
        for &value in slice {
            if value == 0 {
                break; // Null terminator found, stop processing
            }
            result.push(char::from_u32(value as u32).unwrap_or_else(|| {
                // Handle invalid Unicode code points (optional)
                '�'
            }));
        }
        result
    }
}
