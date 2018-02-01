
use std::io::stdout;

mod les;

fn main() {

    let mut std_out_writer = stdout();
    let mut l = les::Les::new(&mut std_out_writer);
    l.run();
}
