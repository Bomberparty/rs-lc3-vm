pub const REG_SIZE: u32 = 10;

pub struct Regs {
    pub r0: u16,
    pub r1: u16,
    pub r2: u16,
    pub r3: u16,
    pub r4: u16,
    pub r5: u16,
    pub r6: u16,
    pub r7: u16,
    pub r_cond: u16,
    pub r_progcount: u16,
}

impl Regs {
    pub fn new() -> Self {
        Regs {
            r0: 0x0,
            r1: 0x0,
            r2: 0x0,
            r3: 0x0,
            r4: 0x0,
            r5: 0x0,
            r6: 0x0,
            r7: 0x0,
            r_cond: 0x0,
            r_progcount: 0x0,
        }
    }

    pub fn get_by_num(&self, addr: u8) -> u16 {
        match addr {
            0x0 => self.r0,
            0x1 => self.r1,
            0x2 => self.r2,
            0x3 => self.r3,
            0x4 => self.r4,
            0x5 => self.r5,
            0x6 => self.r6,
            0x7 => self.r7,
            _ => panic!("Incorrect register is being read!")
        }
    }

    pub fn set_by_num(&mut self, addr: u8, value: u16) {
        match addr {
            0x0 => self.r0 = value,
            0x1 => self.r1 = value,
            0x2 => self.r2 = value,
            0x3 => self.r3 = value,
            0x4 => self.r4 = value,
            0x5 => self.r5 = value,
            0x6 => self.r6 = value,
            0x7 => self.r7 = value,
            _ => panic!("Incorrect register is being read!")
        }
    }
}
