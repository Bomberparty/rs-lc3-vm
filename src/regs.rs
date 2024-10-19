use super::constants::REG_SIZE;

pub struct Regs([u16; REG_SIZE as usize]);

impl Regs {
    pub fn new() -> Self {
        Regs([0; REG_SIZE as usize])
    }

    pub fn get_by_num(&self, addr: u8) -> u16 {
        self.0[addr as usize]
    }

    pub fn set_by_num(&mut self, addr: u8, value: u16) {
        self.0[addr as usize] = value;
    }
}