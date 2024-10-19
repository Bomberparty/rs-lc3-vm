use rs_lc_3_vm::vm::*;

fn main() {
    let mut vm = VM::new();
    vm.regs.r_cond = Flags::FlZro as u16;
    vm.regs.r_progcount = PC_START;
}