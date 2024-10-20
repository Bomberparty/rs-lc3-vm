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

        let start_address = ((buffer[0] as u16) << 8) | buffer[1] as u16;
        let instructions = buffer[2..].chunks_exact(2).map(|chunk| {
            ((chunk[0] as u16) << 8) | chunk[1] as u16
        });

        for (i, instruction) in instructions.enumerate() {
            let address = (start_address as usize + i) % self.memory.len();
            self.memory[address] = instruction;
        }

        Ok(())
    }

    pub fn run(&mut self) {
        let instructions = self.memory.iter().enumerate().map(|(i, &instruction)| {
            (i as u16, instruction)
        });

        for (pc, instruction) in instructions {
            println!("{:04X}", instruction);

            // TODO: Implement the actual VM logic here
        }
    }
}