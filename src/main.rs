mod e_header;
mod linker;
mod utils;
use std::{env::args, fs::File};

use crate::utils::input_file::InputFile;

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        println!("please provide at least one file");
        std::process::exit(1);
    }

    if !std::path::Path::new(&args[1]).exists() {
        println!("cannot find file {}", args[1]);
        std::process::exit(1);
    }

    let file = File::open(&args[1]).expect("Failed to open file");

    let input_file = InputFile::new(file);

    println!(
        "section number of {} is {}",
        args[1],
        input_file.elf_sections.len()
    );
}
