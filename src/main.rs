use std::fs::File;
use std::io::{self, Read};

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

    for chunk in buffer.chunks(2) {
        if chunk.len() == 2 {
            let instruction = ((chunk[0] as u16) << 8) | chunk[1] as u16;
            println!("{:04X}", instruction);
        }
    }

    Ok(())
}