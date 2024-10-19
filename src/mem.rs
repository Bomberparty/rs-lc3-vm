pub const MEM_SIZE: u32 = 1 << 16;

pub struct Mem([u16; MEM_SIZE as usize]);


impl Mem {

    pub fn new() -> Self {
        Mem([0; MEM_SIZE as usize])
    }

    pub fn get_mem(&mut self, addr: usize) -> u16 {
        self.0[addr]
    }

    pub fn set_mem(&mut self, addr: usize, val: u16) -> () {
        self.0[addr] = val
    }
}
