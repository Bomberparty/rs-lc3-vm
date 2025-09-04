use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum VmError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    
    #[error("Invalid register value: {0}")]
    InvalidRegister(u16),
    
    #[error("Invalid trap vector: {0}")]
    InvalidTrapVector(u8),
    
    #[error("Memory access out of bounds: 0x{0:04X}")]
    MemoryOutOfBounds(u16),
    
    #[error("Invalid instruction: 0x{0:04X}")]
    InvalidInstruction(u16),
    
    #[error("Image file too short")]
    ImageTooShort,
    
    #[error("Start address out of bounds: 0x{0:04X}")]
    StartAddressOutOfBounds(u16),
}

pub type Result<T> = std::result::Result<T, VmError>;