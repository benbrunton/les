
extern crate clap; 
extern crate ansi_term;
extern crate terminal_size;
extern crate toml;
extern crate glob;

use std::io::stdout;
use clap::{App, Arg};

mod les;
mod fs;
mod style;
mod io;
mod config;
mod decorate;
mod paintitems;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const DEFAULT_PATH: &str = "./";
const ARGS_FILEPATH: &str = "file";

fn main() {

    let matches = App::new("les")
        .version(VERSION)
        .arg(
            Arg::with_name(ARGS_FILEPATH)
                .help("file or dir to list")
                .index(1)
            )
        .get_matches();


    let dir_option = matches.value_of(ARGS_FILEPATH);
    let dir = dir_option.unwrap_or(DEFAULT_PATH);

    let configuration = config::find_config(dir);

    let decorator = decorate::Decorate::new(Some(&configuration));

    let fs_reader = fs::FsReader;
    let printer = io::TerminalPrinter::new(&decorator);
    let mut l = les::Les::new(dir, &printer, &fs_reader);
    l.run();
}
