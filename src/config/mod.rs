
use std::path::PathBuf;

mod config;
pub use self::config::{Config, Store};

pub fn find_config(target_path: &str) -> Config {

    let absolute_path = PathBuf::from(target_path).canonicalize();
    match absolute_path {
        Err(_)  => return Config::new(""),
        _       => ()
    }

    let mut path = absolute_path.expect("expected to be able to derive absolute path from arg");

    while !check_exists(&path) && path.parent() != None {
        path = (*path.parent().unwrap()).to_path_buf();
    }

    if !check_exists(&path) {
        return Config::new("");
    }

    path.push(".les");
    let p = path.to_str().unwrap_or("");
    Config::new(p)

}

fn check_exists(target_path: &PathBuf) -> bool {
    let mut path = target_path.clone();
    path.push(".les");
    path.exists()
}
