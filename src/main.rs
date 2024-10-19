use rs_lc_3_vm::vm::VM;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to the binary image file
    #[clap(short, long)]
    image: String,
}

fn main() {
    let args = Args::parse();
    let mut vm = VM::new();

    match vm.load_image(&args.image) {
        Ok(_) => {
            println!("Loaded image from: {}", args.image);
            vm.run();
        }
        Err(e) => {
            eprintln!("Failed to load image: {}", e);
        }
    }
}