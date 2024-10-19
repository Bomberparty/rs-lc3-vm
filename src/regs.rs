pub const REG_SIZE: u32 = 10;

pub struct Regs {
    r0: u16,
    r1: u16,
    r2: u16,
    r3: u16,
    r4: u16,
    r5: u16,
    r6: u16,
    r7: u16,
    r_cond: u16,
    r_progcount: u16,
}

impl Regs {
    fn new() -> Self {
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
}
