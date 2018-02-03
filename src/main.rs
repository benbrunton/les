
extern crate clap; 
extern crate ansi_term;
extern crate toml;

use std::io::stdout;
use clap::{App, Arg};

mod les;
mod fs;
mod style;
mod config;

const DEFAULT_PATH: &str = "./";

const ARGS_FILEPATH: &str = "file";

fn main() {

    let matches = App::new("les")
        .version("v0.1-beta")
        .arg(
            Arg::with_name(ARGS_FILEPATH)
                .help("file or dir to list")
                .index(1)
            )
        .get_matches();


    let dir_option = matches.value_of(ARGS_FILEPATH);
    let dir = dir_option.unwrap_or(DEFAULT_PATH);

    config::find_config(dir);

    let mut std_out_writer = stdout();
    let fs_reader = fs::FsReader;
    let mut l = les::Les::new(dir, &mut std_out_writer, &fs_reader);
    l.run();
}
