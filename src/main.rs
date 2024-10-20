mod command;
mod vm;

use std::fs::File;
use std::io::{self, Read};
use vm::VM;

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <path_to_binary_image>", args[0]);
        return Ok(());
    }

    let image_path = &args[1];
    let mut file = File::open(image_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let mut vm = VM::new();
    vm.load_image(&buffer)?;
    vm.run();

    Ok(())
}