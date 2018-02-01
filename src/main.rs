
use std::io::stdout;

mod les;
mod fs;

fn main() {

    let mut std_out_writer = stdout();
    let fs_reader = fs::FsReader;
    let mut l = les::Les::new(&mut std_out_writer, &fs_reader);
    l.run();
}
