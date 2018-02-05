
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

        PaintItem{
            label,
            is_bold: false,
            is_underline: false,
            is_dimmed: false,
            is_hidden: false,
            colour: None,
            icon: None
        }
    }
}
