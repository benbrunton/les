
extern crate clap; 
extern crate ansi_term;
extern crate toml;
extern crate glob;

use std::io::stdout;
use clap::{App, Arg};

mod les;
mod fs;
mod style;
mod config;

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
    let painter = style::Painter::new(configuration);

    let mut std_out_writer = stdout();
    let fs_reader = fs::FsReader;
    let mut l = les::Les::new(dir, &mut std_out_writer, &fs_reader, &painter);
    l.run();
}
