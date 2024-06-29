use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    pub objects: Vec<PathBuf>,

    #[arg(short = 'L', long)]
    pub library_path: Option<Vec<PathBuf>>,

    #[arg(short = 'l', long)]
    pub library: Option<Vec<String>>,

    // can only support elf_x86_64
    #[arg(short = 'm', value_name = "elf_x86_64")]
    pub emulation: Option<String>,

    #[arg(short = 'o', long, default_value = "a.out")]
    pub output: String,

    // the -plugin option cannot be parsed, because short option
    // only accept char
    // but the plugin option is quite useless in linking, so
    // we are safe to ignore this option
    #[arg(short = 'p')]
    pub plugin: Option<String>,
    #[arg(long)]
    pub plugin_opt: Option<Vec<String>>,

    #[arg(long)]
    pub hash_style: Option<String>,

    #[arg(short = 'z')]
    pub keyword: Option<Vec<String>>,

    // clap cannot parse -static, this is a workaround
    #[arg(short = 's', value_name = "tatic")]
    pub static_: Option<String>,
    #[arg(long)]
    pub as_needed: bool,
    #[arg(long)]
    pub build_id: bool,
    #[arg(long)]
    pub start_group: bool,
    #[arg(long)]
    pub end_group: bool,
}
