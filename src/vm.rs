use std::io;

pub struct VM {
    registers: [u16; 8],
    memory: [u16; 65536],
}

impl VM {
    pub fn new() -> Self {
        VM {
            registers: [0; 8],
            memory: [0; 65536],
        }
    }

    pub fn load_image(&mut self, buffer: &[u8]) -> io::Result<()> {
        if buffer.len() < 2 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "File too short"));
        }

        let mut start_address = ((buffer[0] as u16) << 8) | buffer[1] as u16;
        let mut index = 2;

        while index + 1 < buffer.len() {
            let instruction = ((buffer[index] as u16) << 8) | buffer[index + 1] as u16;
            self.memory[start_address as usize] = instruction;
            start_address += 1;
            index += 2;
        }

        Ok(())
    }

    pub fn run(&mut self) {
        let mut pc = 0;

        loop {
            let instruction = self.memory[pc as usize];
            pc += 1;

            // Example: Print the instruction in hexadecimal format
            println!("{:04X}", instruction);

            // TODO: Implement the actual VM logic here
        }
    }
}