
use glob::Pattern;
use toml;
use config::Store;
use fs::File;
use style::PaintItem;

pub struct Decorate<'a>{
    config: Option<&'a Store>
}

impl <'a> Decorate<'a> {

    pub fn new(config: Option<&'a Store>) -> Decorate<'a> {
        Decorate{
            config
        }
    }

    pub fn get_paint_rules(&self, file:&File) -> PaintItem {

        let label = if file.get_is_dir() {
            format!("{}/", file.get_label())
        } else {
            file.get_label()
        };


        let is_hidden = self.is_hidden(&file.get_name());

        PaintItem{
            label,
            is_bold: false,
            is_underline: false,
            is_dimmed: false,
            is_hidden,
            colour: None,
            icon: None
        }
    }

    fn is_hidden(&self, path: &str) -> bool {

        match self.get("hidden") {
            Some(&toml::Value::Array(ref globs)) => {
                for glob in globs {
                    if Pattern::new(glob.as_str().unwrap_or("")).expect("unable to expand glob")
                        .matches(&path) {
                            return true;
                        }
                }

                false
            },
            _ => false
        }
    }

    fn get(&self, key: &'static str) -> Option<&toml::Value> {
        match self.config {
            Some(config) => config.get(key),
            _ => None
        }

    }
}
