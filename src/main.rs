mod command;
mod vm;

use std::fs::File;
use std::io::{self, Read};
use vm::VM;

use clap::{arg, command};

fn main() -> io::Result<()> {
    let matches = command!()
        .about("A simple implementation of LC-3 as a VM")
        .args([
            arg!(-i --image <FILE> "Path to the binary image file").required(true),
            arg!(-d --debug "Switch to a debug mode").required(false),
        ])
        .get_matches();
    let image_path = matches.get_one::<String>("image").expect("image is required");
    let debug_mode = matches.contains_id("debug");

    let mut file = File::open(image_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let mut vm = VM::new();
    vm.load_image(&buffer)?;

    vm.run(&buffer ,debug_mode)?;


    Ok(())
}