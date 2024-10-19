use super::vm::VM;

fn main() {
    let mut vm = VM::new();
    vm.load_image("path/to/your/binary/image.obj").unwrap();
    vm.run();
}