use std::fmt;
use crate::error::{VmError, Result};

/// LC-3 Memory address (16-bit)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Address(pub u16);

impl Address {
    pub const MAX: Address = Address(u16::MAX);
    
    pub fn offset(self, offset: i16) -> Address {
        Address(self.0.wrapping_add_signed(offset as i16))
    }
    
    pub fn usize(self) -> usize {
        self.0 as usize
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:04X}", self.0)
    }
}

impl TryFrom<u16> for Address {
    type Error = VmError;
    
    fn try_from(value: u16) -> Result<Self> {
        Ok(Address(value))
    }
}

/// LC-3 Register identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Register {
    R0 = 0,
    R1 = 1,
    R2 = 2,
    R3 = 3,
    R4 = 4,
    R5 = 5,
    R6 = 6,
    R7 = 7,
}

impl Register {
    pub const COUNT: usize = 8;
    
    pub fn index(self) -> usize {
        self as usize
    }
    
    pub fn all() -> [Register; Self::COUNT] {
        [
            Register::R0,
            Register::R1,
            Register::R2,
            Register::R3,
            Register::R4,
            Register::R5,
            Register::R6,
            Register::R7,
        ]
    }
}

impl TryFrom<u16> for Register {
    type Error = VmError;
    
    fn try_from(value: u16) -> Result<Self> {
        match value {
            0 => Ok(Register::R0),
            1 => Ok(Register::R1),
            2 => Ok(Register::R2),
            3 => Ok(Register::R3),
            4 => Ok(Register::R4),
            5 => Ok(Register::R5),
            6 => Ok(Register::R6),
            7 => Ok(Register::R7),
            _ => Err(VmError::InvalidRegister(value)),
        }
    }
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "R{}", *self as u16)
    }
}

/// LC-3 Condition flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConditionFlag {
    Positive = 0x1,
    Zero = 0x2,
    Negative = 0x4,
}

impl ConditionFlag {
    pub fn from_value(value: u16) -> Self {
        if value == 0 {
            ConditionFlag::Zero
        } else if value & 0x8000 != 0 {
            ConditionFlag::Negative
        } else {
            ConditionFlag::Positive
        }
    }
}

impl fmt::Display for ConditionFlag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConditionFlag::Positive => write!(f, "POS"),
            ConditionFlag::Zero => write!(f, "ZRO"),
            ConditionFlag::Negative => write!(f, "NEG"),
        }
    }
}