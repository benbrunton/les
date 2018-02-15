
use glob::Pattern;
use toml;
use config::Store;
use fs::File;
use style::PaintItem;
use ansi_term::Colour;
use std::collections::BTreeMap;

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
        let colour = match file.get_is_dir() {
            true => self.get_dir_colour(&file.get_name()),
            _    => self.get_colour(&file.get_name())
        };

        PaintItem{
            label,
            is_bold: false,
            is_underline: false,
            is_dimmed: false,
            is_hidden,
            colour: colour,
            icon: None
        }
    }

    fn get_colour(&self, path: &str) -> Option<Colour> {
        match self.get("colour") {

            Some(&toml::Value::Table(ref colour_table)) => {

                for (key, colour) in colour_table {
                    let applied_colour = self.get_colour_from_key(key);

                    if self.matches_glob_array(colour.get("except"), path) {
                        continue;
                    }

                    if self.matches_glob_array(colour.get("colour"), path) {
                        return Some(applied_colour);
                    }
                }

                None
            },
            _ => None
        }
    }

    fn get_dir_colour(&self, path: &str) -> Option<Colour> {

        match self.get("dir") {
            
            Some(&toml::Value::Table(ref dir_table)) => {
                match dir_table.get("colour") {
                    Some(&toml::Value::Table(ref colour_table)) => self.search_colour_table(path, colour_table),
                    _ => None

                }

            },
            _ => None

        }
    }

    fn search_colour_table(&self, path: &str, colour_table: &BTreeMap<String, toml::Value>) -> Option<Colour> {

            for (key, colour) in colour_table {
                let applied_colour = self.get_colour_from_key(key);

                if self.matches_glob_array(colour.get("except"), path) {
                    continue;
                }

                if self.matches_glob_array(colour.get("colour"), path) {
                    return Some(applied_colour);
                }
            }

            None

    }

    fn get_colour_from_key(&self, key: &str) -> Colour {


        match key {
            "black"     => Colour::Black,
            "red"       => Colour::Red,
            "green"     => Colour::Green,
            "yellow"    => Colour::Yellow,
            "blue"      => Colour::Blue,
            "purple"    => Colour::Purple,
            "cyan"      => Colour::Cyan,
            "grey"      => Colour::Fixed(7),
            _           => Colour::White
        }
    }

    fn is_hidden(&self, path: &str) -> bool {

        match self.get("hidden") {

            Some(&toml::Value::Table(ref hidden_table)) => {

                if self.matches_glob_array(hidden_table.get("except"), path) {
                    return false;
                }


                if self.matches_glob_array(hidden_table.get("hidden"), path) {
                    return true;
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


    fn matches_glob_array(&self, glob_array: Option<&toml::Value>, path: &str) -> bool {

        match glob_array {
            Some(&toml::Value::Array(ref globs)) => {
                for glob in globs {
                    if Pattern::new(glob.as_str().unwrap_or(""))
                        .expect("unable to expand glob")
                        .matches(&path) 
                    {
                        return true;
                    }
                }
                false
            },
            _ => false
        }
    }
}

#[cfg(test)]
mod tests {

    use style::PaintItem;
    use config::Store;
    use fs::File;
    use decorate::*;


    #[test]
    fn it_returns_a_Paint_Item() {

        let file = File::new(
            String::from("filename"),
            String::from("file_label"),
            false,
            String::from("/filename")
        );
        let decorator = Decorate::new(None);
        let paint_item = decorator.get_paint_rules(&file);
        let expected_result = PaintItem{
            label: String::from("file_label"),
            is_bold: false,
            is_underline: false,
            is_dimmed: false,
            is_hidden: false,
            colour: None,
            icon: None
        };


        assert_eq!(paint_item, expected_result);
    }

/*    struct MockStore;

    impl Store for MockStore {
        fn get(&self, _: &'static str) -> Option<&toml::Value> {

            None
        }
    }
*/

}

