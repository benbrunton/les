use std::io::prelude::*;
use std::fs::File;
use std::collections::BTreeMap;
use std::string::String;

use toml;

pub trait Store {
    fn get(&self, key: &'static str) -> Option<&toml::Value>;
}

#[derive(Debug)]
pub struct Config{
    store: BTreeMap<String, toml::Value>
}

impl Config {

    pub fn from_path(path: &str) -> Config{

        let f = File::open(path);

        if let Err(_) = f {
            return Config { store: BTreeMap::new() }; 
        };

        let mut file_string = f.unwrap();

        let mut s = String::new();
        let _ = file_string.read_to_string(&mut s);


        Self::from_str(&s)
        
    }

    pub fn from_str(s: &str) -> Config {

        let toml = toml::from_str(&s).unwrap_or(BTreeMap::new());

        Config {
            store : toml
        }

    }

}


impl Store for Config {

    fn get(&self, key: &'static str) -> Option<&toml::Value> {

        self.store.get(key)

    }

}

