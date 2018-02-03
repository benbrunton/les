
use std::path::PathBuf;

mod config;

pub fn find_config(target_path: &str) -> Option<config::Config> {

    let absolute_path = PathBuf::from(target_path).canonicalize();
    match absolute_path {
        Err(_)  => return None,
        _       => ()
    }

    let mut path = absolute_path.expect("expected to be able to derive absolute path from arg");

    while !check_exists(&path) && path.parent() != None {
        path = (*path.parent().unwrap()).to_path_buf();
    }

    if !check_exists(&path) {
        return None;
    }

    path.push(".les");
    let c = config::Config::new(path.to_str().expect("unable to set path to string"));
    Some(c)

}

fn check_exists(target_path: &PathBuf) -> bool {
    let mut path = target_path.clone();
    path.push(".les");
    path.exists()
}
