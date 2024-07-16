mod archive_parser;
mod argument_parser;
mod e_header;
mod linker;
mod section_header;
mod utils;
use std::{env::args, fs::File};

use argument_parser::Args;
use clap::Parser;

use crate::utils::input_file::ElfData;

fn main() {
    let args = Args::parse();
    // let args: Vec<String> = args().collect();
    // dbg!(&args);
    let archive_parser = archive_parser::Parser::new(args.library_path);

    let mut elf_data = vec![];
    if let Some(library) = args.library {
        for archive in library {
            let mut elf = archive_parser.parse(archive);
            elf_data.append(&mut elf);
        }
    }

    let elf_size = elf_data.len();
    dbg!(elf_size);
    for i in 1..10 {
        let name = &elf_data[elf_size - i].name;
        dbg!(name);
    }

    return;

    // TODO: check every input file is relocatable elf

    // dbg!(&args);

    // if args.len() < 2 {
    //     println!("please provide at least one file");
    //     std::process::exit(1);
    // }

    // if !std::path::Path::new(&args[1]).exists() {
    //     println!("cannot find file {}", args[1]);
    //     std::process::exit(1);
    // }

    // let file = File::open(&args[1]).expect("Failed to open file");

    // let input_file = InputFile::new(file);

    // println!(
    //     "section number of {} is {}",
    //     args[1],
    //     input_file.elf_sections.len()
    // );

    // print!("{}", input_file);
}
