pub enum Trap {
    TrapGetc = 0x20,  /* get character from keyboard, not echoed onto the terminal */
    TrapOut = 0x21,   /* output a character */
    TrapPuts = 0x22,  /* output a word string */
    TrapIn = 0x23,    /* get character from keyboard, echoed onto the terminal */
    TrapPutsp = 0x24, /* output a byte string */
    TrapHalt = 0x25,  /* halt the program */
}

impl Trap {
    pub fn get_by_num(value: u16) -> Trap {
        match value {
            0x20 => Trap::TrapGetc,
            0x21 => Trap::TrapOut,
            0x22 => Trap::TrapPuts,
            0x23 => Trap::TrapIn,
            0x24 => Trap::TrapPutsp,
            0x25 => Trap::TrapHalt,
            _ => panic!("Incorrect trap routine number!"),
        }
    }
}