use rs_lc_3_vm::vm::VM;

fn main() {
    let mut vm = VM::new();
    vm.load_image("path/to/your/binary/image.obj").unwrap();
    vm.run();
}