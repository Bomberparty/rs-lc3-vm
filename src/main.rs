use clap::{Arg, Command};
use rs_lc_3_vm::{Vm, Result};
use std::fs::File;
use std::io::Read;

fn main() -> Result<()> {
    let matches = Command::new("rs-lc-3-vm")
        .version("0.2.3")
        .author("Your Name")
        .about("LC-3 Virtual Machine implementation in Rust")
        .arg(
            Arg::new("image")
                .short('i')
                .long("image")
                .value_name("FILE")
                .help("Path to the binary image file")
                .required(true),
        )
        .arg(
            Arg::new("debug")
                .short('d')
                .long("debug")
                .help("Enable debug mode")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let image_path = matches.get_one::<String>("image").expect("required");
    let debug_mode = matches.get_flag("debug");

    // Read image file
    let mut file = File::open(image_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // Create and run VM
    let mut vm = Vm::new();
    vm.load_image(&buffer)?;
    vm.run(debug_mode)?;

    Ok(())
}