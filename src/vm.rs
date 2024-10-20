use std::io;
use super::command::Command;

pub struct VM {
    registers: [u16; 8],
    flags: u16, // 3 flags: FlPos (0x0), FlZro (0x1), FlNeg (0x2)
    memory: [u16; 65536],
}

impl VM {
    pub fn new() -> Self {
        VM {
            registers: [0; 8],
            flags: 0,
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
                return Err(io::Error::new(io::ErrorKind::InvalidData, "Start address out of bounds"));
            }

            let instruction = ((buffer[index] as u16) << 8) | buffer[index + 1] as u16;
            self.memory[(start_address as usize + (index - 2) / 2)] = instruction;
            index += 2;
        }

        Ok(())
    }

    pub fn run(&mut self, buffer: &[u8]) {
        let start_address = self.extract_start_address(buffer);
        let mut pc = start_address; 

        loop {
            if pc as usize >= self.memory.len() {
                break; 
            }

            let instruction = self.memory[pc as usize];
            let command = Command::from_u16(instruction);
            pc = (pc + 1)%u16::MAX;
        }
    }

    fn extract_start_address(&self, buffer: &[u8]) -> u16 {
        ((buffer[0] as u16) << 8) | buffer[1] as u16
    }

    fn update_flags(&mut self, value: u16) {
        if value == 0 {
            self.flags = 0x1; // FlZro
        } else if value & 0x8000 != 0 {
            self.flags = 0x2; // FlNeg
        } else {
            self.flags = 0x0; // FlPos
        }
    }
}