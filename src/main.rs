use std::io::stdout;

extern crate ansi_term;

mod les;
mod fs;
mod style;

fn main() {

    let mut std_out_writer = stdout();
    let fs_reader = fs::FsReader;
    let mut l = les::Les::new(&mut std_out_writer, &fs_reader);
    l.run();
}
