use crate::{
    command::{Command, TrapVector},
    error::{VmError, Result},
    memory::Memory,
    types::{Address, ConditionFlag, Register},
};
use std::io::{self, Read, Write};

/// LC-3 Virtual Machine
pub struct Vm {
    registers: [u16; Register::COUNT],
    condition_flag: ConditionFlag,
    memory: Memory,
    program_counter: Address,
    running: bool,
}

impl Vm {
    pub fn new() -> Self {
        Vm {
            registers: [0; Register::COUNT],
            condition_flag: ConditionFlag::Zero,
            memory: Memory::new(),
            program_counter: Address(0x3000), // Default start address
            running: false,
        }
    }
    
    pub fn load_image(&mut self, image_data: &[u8]) -> Result<()> {
        if image_data.len() < 2 {
            return Err(VmError::ImageTooShort);
        }
        
        // Extract start address from first two bytes
        let start_address = Address(u16::from_be_bytes([image_data[0], image_data[1]]));
        self.program_counter = start_address;
        
        // Load the rest of the image
        self.memory.load_image(start_address, &image_data[2..])
    }
    
    pub fn run(&mut self, debug_mode: bool) -> Result<()> {
        self.running = true;
        
        if debug_mode {
            println!("Starting execution from address {}", self.program_counter);
            println!("{}", self.memory);
        }
        
        while self.running {
            self.execute_instruction(debug_mode)?;
        }
        
        Ok(())
    }
    
    fn execute_instruction(&mut self, debug_mode: bool) -> Result<()> {
        // Fetch instruction
        let instruction = self.memory.read(self.program_counter)?;
        let next_pc = Address(self.program_counter.0.wrapping_add(1));
        
        if debug_mode {
            println!("\n--- Execution Step ---");
            println!("PC: {}", self.program_counter);
            println!("Instruction: 0x{:04X}", instruction);
            self.print_registers();
        }
        
        // Decode and execute
        let command = Command::parse(instruction)?;
        
        if debug_mode {
            println!("Command: {}", command);
        }
        
        match command {
            Command::Add { dr, sr1, sr2, imm5 } => self.execute_add(dr, sr1, sr2, imm5),
            Command::And { dr, sr1, sr2, imm5 } => self.execute_and(dr, sr1, sr2, imm5),
            Command::Branch { condition, offset } => self.execute_branch(condition, offset),
            Command::Jump { base_r } => self.execute_jump(base_r),
            Command::JumpSubroutine { offset, base_r } => self.execute_jump_subroutine(offset, base_r),
            Command::Load { dr, offset } => self.execute_load(dr, offset),
            Command::LoadIndirect { dr, offset } => self.execute_load_indirect(dr, offset),
            Command::LoadRegister { dr, base_r, offset } => self.execute_load_register(dr, base_r, offset),
            Command::LoadEffectiveAddress { dr, offset } => self.execute_load_effective_address(dr, offset),
            Command::Not { dr, sr } => self.execute_not(dr, sr),
            Command::ReturnFromInterrupt => self.execute_return_from_interrupt(),
            Command::Store { sr, offset } => self.execute_store(sr, offset),
            Command::StoreIndirect { sr, offset } => self.execute_store_indirect(sr, offset),
            Command::StoreRegister { sr, base_r, offset } => self.execute_store_register(sr, base_r, offset),
            Command::Trap { vector } => self.execute_trap(vector),
            Command::Reserved => Err(VmError::InvalidInstruction(instruction)),
        }?;
        
        // Update program counter if it wasn't modified by the instruction
        self.program_counter = next_pc;
        
        if debug_mode {
            println!("--- End Step ---");
        }
        
        Ok(())
    }
    
    // Instruction implementations
    fn execute_add(&mut self, dr: Register, sr1: Register, sr2: Option<Register>, imm5: Option<u16>) -> Result<()> {
        let value1 = self.registers[sr1.index()];
        let value2 = if let Some(sr2) = sr2 {
            self.registers[sr2.index()]
        } else if let Some(imm5) = imm5 {
            imm5
        } else {
            return Err(VmError::InvalidInstruction(0)); // Shouldn't happen
        };
        
        let result = value1.wrapping_add(value2);
        self.registers[dr.index()] = result;
        self.update_condition_flag(result);
        Ok(())
    }
    
    fn execute_and(&mut self, dr: Register, sr1: Register, sr2: Option<Register>, imm5: Option<u16>) -> Result<()> {
        let value1 = self.registers[sr1.index()];
        let value2 = if let Some(sr2) = sr2 {
            self.registers[sr2.index()]
        } else if let Some(imm5) = imm5 {
            imm5
        } else {
            return Err(VmError::InvalidInstruction(0)); // Shouldn't happen
        };
        
        let result = value1 & value2;
        self.registers[dr.index()] = result;
        self.update_condition_flag(result);
        Ok(())
    }
    
    fn execute_branch(&mut self, condition: u16, offset: i16) -> Result<()> {
        if condition & self.condition_flag as u16 != 0 {
            self.program_counter = self.program_counter.offset(offset);
        }
        Ok(())
    }
    
    fn execute_jump(&mut self, base_r: Register) -> Result<()> {
        self.program_counter = Address(self.registers[base_r.index()]);
        Ok(())
    }
    
    fn execute_jump_subroutine(&mut self, offset: Option<i16>, base_r: Option<Register>) -> Result<()> {
        // Save return address in R7
        self.registers[Register::R7.index()] = self.program_counter.0;
        
        if let Some(offset) = offset {
            self.program_counter = self.program_counter.offset(offset);
        } else if let Some(base_r) = base_r {
            self.program_counter = Address(self.registers[base_r.index()]);
        }
        
        Ok(())
    }
    
    fn execute_load(&mut self, dr: Register, offset: i16) -> Result<()> {
        let address = self.program_counter.offset(offset);
        let value = self.memory.read(address)?;
        self.registers[dr.index()] = value;
        self.update_condition_flag(value);
        Ok(())
    }
    
    fn execute_load_indirect(&mut self, dr: Register, offset: i16) -> Result<()> {
        let address_ptr = self.program_counter.offset(offset);
        let address = Address(self.memory.read(address_ptr)?);
        let value = self.memory.read(address)?;
        self.registers[dr.index()] = value;
        self.update_condition_flag(value);
        Ok(())
    }
    
    fn execute_load_register(&mut self, dr: Register, base_r: Register, offset: i16) -> Result<()> {
        let base_address = self.registers[base_r.index()];
        let address = Address(base_address).offset(offset);
        let value = self.memory.read(address)?;
        self.registers[dr.index()] = value;
        self.update_condition_flag(value);
        Ok(())
    }
    
    fn execute_load_effective_address(&mut self, dr: Register, offset: i16) -> Result<()> {
        let address = self.program_counter.offset(offset);
        self.registers[dr.index()] = address.0;
        self.update_condition_flag(address.0);
        Ok(())
    }
    
    fn execute_not(&mut self, dr: Register, sr: Register) -> Result<()> {
        let value = !self.registers[sr.index()];
        self.registers[dr.index()] = value;
        self.update_condition_flag(value);
        Ok(())
    }
    
    fn execute_return_from_interrupt(&mut self) -> Result<()> {
        // For now, just halt execution
        self.running = false;
        Ok(())
    }
    
    fn execute_store(&mut self, sr: Register, offset: i16) -> Result<()> {
        let address = self.program_counter.offset(offset);
        let value = self.registers[sr.index()];
        self.memory.write(address, value)?;
        Ok(())
    }
    
    fn execute_store_indirect(&mut self, sr: Register, offset: i16) -> Result<()> {
        let address_ptr = self.program_counter.offset(offset);
        let address = Address(self.memory.read(address_ptr)?);
        let value = self.registers[sr.index()];
        self.memory.write(address, value)?;
        Ok(())
    }
    
    fn execute_store_register(&mut self, sr: Register, base_r: Register, offset: i16) -> Result<()> {
        let base_address = self.registers[base_r.index()];
        let address = Address(base_address).offset(offset);
        let value = self.registers[sr.index()];
        self.memory.write(address, value)?;
        Ok(())
    }
    
    fn execute_trap(&mut self, vector: TrapVector) -> Result<()> {
        match vector {
            TrapVector::GetC => self.trap_getc(),
            TrapVector::Out => self.trap_out(),
            TrapVector::Puts => self.trap_puts(),
            TrapVector::In => self.trap_in(),
            TrapVector::PutSp => self.trap_putsp(),
            TrapVector::Halt => self.trap_halt(),
        }
    }
    
    // TRAP routines
    fn trap_getc(&mut self) -> Result<()> {
        let mut buffer = [0u8; 1];
        io::stdin().read_exact(&mut buffer)?;
        self.registers[Register::R0.index()] = buffer[0] as u16;
        Ok(())
    }
    
    fn trap_out(&mut self) -> Result<()> {
        let c = self.registers[Register::R0.index()] as u8 as char;
        print!("{}", c);
        io::stdout().flush()?;
        Ok(())
    }
    
    fn trap_puts(&mut self) -> Result<()> {
        let mut address = Address(self.registers[Register::R0.index()]);
        
        loop {
            let c = self.memory.read(address)?;
            if c == 0 {
                break;
            }
            print!("{}", c as u8 as char);
            address = Address(address.0 + 1);
        }
        
        io::stdout().flush()?;
        Ok(())
    }
    
    fn trap_in(&mut self) -> Result<()> {
        print!("Enter a character: ");
        io::stdout().flush()?;
        
        let mut buffer = [0u8; 1];
        io::stdin().read_exact(&mut buffer)?;
        
        self.registers[Register::R0.index()] = buffer[0] as u16;
        Ok(())
    }
    
    fn trap_putsp(&mut self) -> Result<()> {
        let mut address = Address(self.registers[Register::R0.index()]);
        
        loop {
            let c = self.memory.read(address)?;
            if c == 0 {
                break;
            }
            
            // Print both bytes packed in the word
            let c1 = (c & 0xFF) as u8 as char;
            let c2 = (c >> 8) as u8 as char;
            
            print!("{}{}", c1, c2);
            address = Address(address.0 + 1);
        }
        
        io::stdout().flush()?;
        Ok(())
    }
    
    fn trap_halt(&mut self) -> Result<()> {
        println!("HALT instruction executed");
        self.running = false;
        Ok(())
    }
    
    // Helper methods
    fn update_condition_flag(&mut self, value: u16) {
        self.condition_flag = ConditionFlag::from_value(value);
    }
    
    fn print_registers(&self) {
        println!("Registers:");
        for reg in Register::all() {
            println!(
                "  {}: 0x{:04X} ({})", 
                reg, 
                self.registers[reg.index()], 
                self.registers[reg.index()]
            );
        }
        println!("Condition Flag: {}", self.condition_flag);
    }
}

impl Default for Vm {
    fn default() -> Self {
        Self::new()
    }
}