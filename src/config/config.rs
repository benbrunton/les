
use std::io::prelude::*;
use std::fs::File;
use std::collections::BTreeMap;
use std::string::String;
use std::env;

use toml;

#[derive(Debug)]
pub struct Config{
    store: BTreeMap<String, toml::Value>
}

impl Config {

    pub fn new(path: &str) -> Config{

        let mut f = File::open(path).expect("unable to open .les");
        let mut s = String::new();
        let _ = f.read_to_string(&mut s);

        let toml = toml::from_str(&s).expect("unable to parse .les");

        Config {
            store : toml
        }

    }

    pub fn get(&self, key: &'static str) -> Option<String> {

        match self.store.get(key){
            Some(val) => {
                let new_str = val.to_string();
                let trimmed_str = new_str.trim_matches('"').to_string();
                Some(trimmed_str)
            },
            None => None
        }

    }
}
