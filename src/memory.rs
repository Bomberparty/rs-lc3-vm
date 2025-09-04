use crate::{
    error::{VmError, Result},
    types::Address,
};
use std::fmt;

/// LC-3 Memory (65,536 16-bit words)
#[derive(Debug)]
pub struct Memory {
    data: [u16; Self::SIZE],
}

impl Memory {
    pub const SIZE: usize = 65536;
    pub const MAX_ADDRESS: Address = Address((Self::SIZE - 1) as u16);
    
    pub fn new() -> Self {
        Memory { data: [0; Self::SIZE] }
    }
    
    pub fn read(&self, address: Address) -> Result<u16> {
        self.validate_address(address)?;
        Ok(self.data[address.usize()])
    }
    
    pub fn write(&mut self, address: Address, value: u16) -> Result<()> {
        self.validate_address(address)?;
        self.data[address.usize()] = value;
        Ok(())
    }
    
    pub fn read_range(&self, start: Address, count: usize) -> Result<&[u16]> {
        let start_idx = start.usize();
        let end_idx = start_idx + count;
        
        if end_idx > Self::SIZE {
            return Err(VmError::MemoryOutOfBounds(end_idx as u16));
        }
        
        Ok(&self.data[start_idx..end_idx])
    }
    
    pub fn load_image(&mut self, start_address: Address, image_data: &[u8]) -> Result<()> {
        if image_data.len() % 2 != 0 {
            return Err(VmError::ImageTooShort);
        }
        
        let start_idx = start_address.usize();
        let word_count = image_data.len() / 2;
        let end_idx = start_idx + word_count;
        
        if end_idx > Self::SIZE {
            return Err(VmError::StartAddressOutOfBounds(start_address.0));
        }
        
        for (i, chunk) in image_data.chunks_exact(2).enumerate() {
            let value = u16::from_be_bytes([chunk[0], chunk[1]]);
            self.data[start_idx + i] = value;
        }
        
        Ok(())
    }
    
    fn validate_address(&self, address: Address) -> Result<()> {
        if address.0 as usize >= Self::SIZE {
            Err(VmError::MemoryOutOfBounds(address.0))
        } else {
            Ok(())
        }
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Memory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Display a summary of memory usage
        let used_memory = self.data.iter().filter(|&&x| x != 0).count();
        write!(
            f,
            "Memory: {}/{} words used ({:.1}%)",
            used_memory,
            Self::SIZE,
            (used_memory as f32 / Self::SIZE as f32) * 100.0
        )
    }
}