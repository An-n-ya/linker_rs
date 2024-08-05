mod archive_parser;
mod argument_parser;
mod context;
mod e_header;
mod linker;
mod output_section;
mod passes;
mod section;
mod symbol;
mod utils;
use std::{fs::File, rc::Rc, sync::Mutex};

use argument_parser::Args;
use clap::Parser;
use context::Context;
use linker::SectionFlag;
use utils::input_elf::InputElf;

pub type Id = Rc<Mutex<usize>>;

fn main() {
    let args = Args::parse();
    // let args: Vec<String> = args().collect();
    // dbg!(&args);
    let archive_parser = archive_parser::Parser::new(args.library_path);

    let mut ctx = Context::new();

    if let Some(library) = args.library {
        for archive in library {
            let elf = archive_parser.parse(archive);
            for obj in elf {
                ctx.push(obj);
            }
        }
    }

    for obj_path in args.objects {
        let f = File::open(&obj_path).expect(&format!("cannot open file {:?}", &obj_path));
        let mut elf = InputElf::new(
            f,
            obj_path.file_name().unwrap().to_str().unwrap().to_string(),
        );
        elf.is_alive = true;
        if elf.name == "a.o" {
            println!("name: {} \n {elf}", elf.name);
        }
        ctx.push(elf);
    }

    let elf_size = ctx.obj_size();
    dbg!(elf_size);

    ctx.resolve_symbol();

    for elf in ctx.object_iter() {
        let elf = elf.lock().unwrap();
        if elf.name != "a.o" {
            let mut merge_sec_ind = vec![];
            for (i, sec) in elf.section_info.elf_sections.iter().enumerate() {
                if sec.flags & SectionFlag::MERGE as u64 != 0 {
                    merge_sec_ind.push(i);
                }
            }
            for ind in merge_sec_ind {
                if let Some(sec) = &elf.section_info.sections[ind] {
                    println!("({})[{}] {:?}", elf.name, sec.name, sec.data);
                }
            }
            // if let Some(info) = &elf.symbol_info {
            //     for sym in &info.global_symbols {
            //         let sym = sym.lock().unwrap();
            //         if sym.name == "puts" {
            //             let target_elf = ctx.get_object(sym.elf.unwrap()).unwrap();
            //             let target_elf = target_elf.lock().unwrap();
            //             println!("puts method is in {}", target_elf.name);
            //         }
            //     }
            // }
        }
    }

    return;
}
