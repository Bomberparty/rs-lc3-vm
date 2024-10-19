pub enum Opcode {
    OpBR = 0x0,   /* branch */
    OpADD = 0x1,  /* add  */
    OpLD = 0x2,   /* load */
    OpST = 0x3,   /* store */
    OpJSR = 0x4,  /* jump register */
    OpAND = 0x5,  /* bitwise and */
    OpLDR = 0x6,  /* load register */
    OpSTR = 0x7,  /* store register */
    OpRTI = 0x8,  /* unused */
    OpNOT = 0x9,  /* bitwise not */
    OpLDI = 0xA,  /* load indirect */
    OpSTI = 0xB,  /* store indirect */
    OpJMP = 0xC,  /* jump */
    OpRES = 0xD,  /* reserved (unused) */
    OpLEA = 0xE,  /* load effective address */
    OpTRAP = 0xF, /* execute trap */
}

pub enum Flags {
    FlPos = 0x1,
    FlZro = 0x2,
    FlNeg = 0x4,
}

pub enum Trap {
    TrapGetc = 0x20,  /* get character from keyboard, not echoed onto the terminal */
    TrapOut = 0x21,   /* output a character */
    TrapPuts = 0x22,  /* output a word string */
    TrapIn = 0x23,    /* get character from keyboard, echoed onto the terminal */
    TrapPutsp = 0x24, /* output a byte string */
    TrapHalt = 0x25,  /* halt the program */
}