
extern crate clap; 
extern crate ansi_term;

use std::io::stdout;
use clap::{App, Arg};

mod les;
mod fs;
mod style;

fn main() {

    let matches = App::new("les")
        .version("v0.1-beta")
        .arg(
            Arg::with_name("file")
                .help("file or dir to list")
                .required(false)
                .index(1)
            )
        .get_matches();


    let dir_option = matches.value_of("file");
    let dir = dir_option.unwrap_or("./");

    let mut std_out_writer = stdout();
    let fs_reader = fs::FsReader;
    let mut l = les::Les::new(dir, &mut std_out_writer, &fs_reader);
    l.run();
}
