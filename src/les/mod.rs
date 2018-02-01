// les module
use std::fs;
use std::path::Path;
use std::io::Write;

pub struct Les</*'args,*/ 'w, W: Write + 'w> {

    pub writer: &'w mut W,

//    pub args: Vec<&'args OsStr>
}


impl</*'args,*/ 'w, W: Write + 'w> Les</*'args,*/ 'w, W> {

    pub fn new(writer: &'w mut W) -> Les<'w, W> {

        Les { writer }
    }

    pub fn run(&mut self){
        let paths = fs::read_dir("./").unwrap();

        for path in paths {
            let dir_entry = path.unwrap();
            let meta = dir_entry.metadata().unwrap();
            let slash = if meta.is_dir() { "/" } else { "" };
            let dir_path = dir_entry.path();
            let stem_option = dir_path.file_name();
            let stem = stem_option.unwrap();
            let p = Path::new(stem);
            let _ = writeln!(self.writer, "{}{}", p.to_str().unwrap(), slash);
        }

    }
}
