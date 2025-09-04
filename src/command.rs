use std::fmt;
use crate::{
    error::{VmError, Result},
    types::{Register},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TrapVector {
    GetC = 0x20,
    Out = 0x21,
    Puts = 0x22,
    In = 0x23,
    PutSp = 0x24,
    Halt = 0x25,
}

impl TryFrom<u8> for TrapVector {
    type Error = VmError;
    
    fn try_from(value: u8) -> Result<Self> {
        match value {
            0x20 => Ok(TrapVector::GetC),
            0x21 => Ok(TrapVector::Out),
            0x22 => Ok(TrapVector::Puts),
            0x23 => Ok(TrapVector::In),
            0x24 => Ok(TrapVector::PutSp),
            0x25 => Ok(TrapVector::Halt),
            _ => Err(VmError::InvalidTrapVector(value)),
        }
    }
}

impl fmt::Display for TrapVector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TrapVector::GetC => write!(f, "GETC"),
            TrapVector::Out => write!(f, "OUT"),
            TrapVector::Puts => write!(f, "PUTS"),
            TrapVector::In => write!(f, "IN"),
            TrapVector::PutSp => write!(f, "PUTSP"),
            TrapVector::Halt => write!(f, "HALT"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Command {
    Add {
        dr: Register,
        sr1: Register,
        sr2: Option<Register>,
        imm5: Option<u16>,
    },
    And {
        dr: Register,
        sr1: Register,
        sr2: Option<Register>,
        imm5: Option<u16>,
    },
    Branch {
        condition: u16,
        offset: i16,
    },
    Jump {
        base_r: Register,
    },
    JumpSubroutine {
        offset: Option<i16>,
        base_r: Option<Register>,
    },
    Load {
        dr: Register,
        offset: i16,
    },
    LoadIndirect {
        dr: Register,
        offset: i16,
    },
    LoadRegister {
        dr: Register,
        base_r: Register,
        offset: i16,
    },
    LoadEffectiveAddress {
        dr: Register,
        offset: i16,
    },
    Not {
        dr: Register,
        sr: Register,
    },
    ReturnFromInterrupt,
    Store {
        sr: Register,
        offset: i16,
    },
    StoreIndirect {
        sr: Register,
        offset: i16,
    },
    StoreRegister {
        sr: Register,
        base_r: Register,
        offset: i16,
    },
    Trap {
        vector: TrapVector,
    },
    Reserved,
}

impl Command {
    pub fn parse(instruction: u16) -> Result<Self> {
        let opcode = instruction >> 12;
        
        match opcode {
            0x1 => Self::parse_add(instruction),
            0x5 => Self::parse_and(instruction),
            0x0 => Self::parse_branch(instruction),
            0xC => Self::parse_jump(instruction),
            0x4 => Self::parse_jump_subroutine(instruction),
            0x2 => Self::parse_load(instruction),
            0xA => Self::parse_load_indirect(instruction),
            0x6 => Self::parse_load_register(instruction),
            0xE => Self::parse_load_effective_address(instruction),
            0x9 => Self::parse_not(instruction),
            0x8 => Ok(Command::ReturnFromInterrupt),
            0x3 => Self::parse_store(instruction),
            0xB => Self::parse_store_indirect(instruction),
            0x7 => Self::parse_store_register(instruction),
            0xF => Self::parse_trap(instruction),
            _ => Ok(Command::Reserved),
        }
    }
    
    fn parse_add(instruction: u16) -> Result<Self> {
        let dr = Register::try_from((instruction >> 9) & 0x7)?;
        let sr1 = Register::try_from((instruction >> 6) & 0x7)?;
        
        if instruction & 0x20 != 0 {
            // Immediate mode
            let imm5 = instruction & 0x1F;
            Ok(Command::Add {
                dr,
                sr1,
                sr2: None,
                imm5: Some(imm5),
            })
        } else {
            // Register mode
            let sr2 = Register::try_from(instruction & 0x7)?;
            Ok(Command::Add {
                dr,
                sr1,
                sr2: Some(sr2),
                imm5: None,
            })
        }
    }
    
    fn parse_and(instruction: u16) -> Result<Self> {
        let dr = Register::try_from((instruction >> 9) & 0x7)?;
        let sr1 = Register::try_from((instruction >> 6) & 0x7)?;
        
        if instruction & 0x20 != 0 {
            // Immediate mode
            let imm5 = instruction & 0x1F;
            Ok(Command::And {
                dr,
                sr1,
                sr2: None,
                imm5: Some(imm5),
            })
        } else {
            // Register mode
            let sr2 = Register::try_from(instruction & 0x7)?;
            Ok(Command::And {
                dr,
                sr1,
                sr2: Some(sr2),
                imm5: None,
            })
        }
    }
    
    fn parse_branch(instruction: u16) -> Result<Self> {
        let condition = (instruction >> 9) & 0x7;
        let offset = sign_extend(instruction & 0x1FF, 9);
        Ok(Command::Branch { condition, offset })
    }
    
    fn parse_jump(instruction: u16) -> Result<Self> {
        let base_r = Register::try_from((instruction >> 6) & 0x7)?;
        Ok(Command::Jump { base_r })
    }
    
    fn parse_jump_subroutine(instruction: u16) -> Result<Self> {
        if instruction & 0x0800 != 0 {
            // JSR
            let offset = sign_extend(instruction & 0x7FF, 11);
            Ok(Command::JumpSubroutine {
                offset: Some(offset),
                base_r: None,
            })
        } else {
            // JSRR
            let base_r = Register::try_from((instruction >> 6) & 0x7)?;
            Ok(Command::JumpSubroutine {
                offset: None,
                base_r: Some(base_r),
            })
        }
    }
    
    fn parse_load(instruction: u16) -> Result<Self> {
        let dr = Register::try_from((instruction >> 9) & 0x7)?;
        let offset = sign_extend(instruction & 0x1FF, 9);
        Ok(Command::Load { dr, offset })
    }
    
    fn parse_load_indirect(instruction: u16) -> Result<Self> {
        let dr = Register::try_from((instruction >> 9) & 0x7)?;
        let offset = sign_extend(instruction & 0x1FF, 9);
        Ok(Command::LoadIndirect { dr, offset })
    }
    
    fn parse_load_register(instruction: u16) -> Result<Self> {
        let dr = Register::try_from((instruction >> 9) & 0x7)?;
        let base_r = Register::try_from((instruction >> 6) & 0x7)?;
        let offset = sign_extend(instruction & 0x3F, 6);
        Ok(Command::LoadRegister {
            dr,
            base_r,
            offset,
        })
    }
    
    fn parse_load_effective_address(instruction: u16) -> Result<Self> {
        let dr = Register::try_from((instruction >> 9) & 0x7)?;
        let offset = sign_extend(instruction & 0x1FF, 9);
        Ok(Command::LoadEffectiveAddress { dr, offset })
    }
    
    fn parse_not(instruction: u16) -> Result<Self> {
        let dr = Register::try_from((instruction >> 9) & 0x7)?;
        let sr = Register::try_from((instruction >> 6) & 0x7)?;
        Ok(Command::Not { dr, sr })
    }
    
    fn parse_store(instruction: u16) -> Result<Self> {
        let sr = Register::try_from((instruction >> 9) & 0x7)?;
        let offset = sign_extend(instruction & 0x1FF, 9);
        Ok(Command::Store { sr, offset })
    }
    
    fn parse_store_indirect(instruction: u16) -> Result<Self> {
        let sr = Register::try_from((instruction >> 9) & 0x7)?;
        let offset = sign_extend(instruction & 0x1FF, 9);
        Ok(Command::StoreIndirect { sr, offset })
    }
    
    fn parse_store_register(instruction: u16) -> Result<Self> {
        let sr = Register::try_from((instruction >> 9) & 0x7)?;
        let base_r = Register::try_from((instruction >> 6) & 0x7)?;
        let offset = sign_extend(instruction & 0x3F, 6);
        Ok(Command::StoreRegister {
            sr,
            base_r,
            offset,
        })
    }
    
    fn parse_trap(instruction: u16) -> Result<Self> {
        let vector = TrapVector::try_from((instruction & 0xFF) as u8)?;
        Ok(Command::Trap { vector })
    }
}

/// Sign-extend a value with the given number of bits
fn sign_extend(mut x: u16, bit_count: u32) -> i16 {
    if (x >> (bit_count - 1)) & 1 != 0 {
        x |= 0xFFFF << bit_count;
    }
    x as i16
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Command::Add { dr, sr1, sr2, imm5 } => {
                write!(f, "ADD {}, {}", dr, sr1)?;
                if let Some(sr2) = sr2 {
                    write!(f, ", {}", sr2)
                } else if let Some(imm5) = imm5 {
                    write!(f, ", #{}", imm5)
                } else {
                    Ok(())
                }
            }
            Command::And { dr, sr1, sr2, imm5 } => {
                write!(f, "AND {}, {}", dr, sr1)?;
                if let Some(sr2) = sr2 {
                    write!(f, ", {}", sr2)
                } else if let Some(imm5) = imm5 {
                    write!(f, ", #{}", imm5)
                } else {
                    Ok(())
                }
            }
            Command::Branch { condition, offset } => {
                write!(f, "BR")?;
                if *condition & 0x4 != 0 { write!(f, "n")?; }
                if *condition & 0x2 != 0 { write!(f, "z")?; }
                if *condition & 0x1 != 0 { write!(f, "p")?; }
                write!(f, " {}", offset)
            }
            Command::Jump { base_r } => write!(f, "JMP {}", base_r),
            Command::JumpSubroutine { offset, base_r } => {
                write!(f, "JSR")?;
                if let Some(offset) = offset {
                    write!(f, " {}", offset)
                } else if let Some(base_r) = base_r {
                    write!(f, "R {}", base_r)
                } else {
                    Ok(())
                }
            }
            Command::Load { dr, offset } => write!(f, "LD {}, {}", dr, offset),
            Command::LoadIndirect { dr, offset } => write!(f, "LDI {}, {}", dr, offset),
            Command::LoadRegister { dr, base_r, offset } => {
                write!(f, "LDR {}, {}, {}", dr, base_r, offset)
            }
            Command::LoadEffectiveAddress { dr, offset } => write!(f, "LEA {}, {}", dr, offset),
            Command::Not { dr, sr } => write!(f, "NOT {}, {}", dr, sr),
            Command::ReturnFromInterrupt => write!(f, "RTI"),
            Command::Store { sr, offset } => write!(f, "ST {}, {}", sr, offset),
            Command::StoreIndirect { sr, offset } => write!(f, "STI {}, {}", sr, offset),
            Command::StoreRegister { sr, base_r, offset } => {
                write!(f, "STR {}, {}, {}", sr, base_r, offset)
            }
            Command::Trap { vector } => write!(f, "TRAP {}", vector),
            Command::Reserved => write!(f, "RES"),
        }
    }
}