// les module
use std::io::Write;
use fs::DirReader;
use decorate::Decorate;
use style::paint;

pub struct Les<'a, W: Write + 'a, R: DirReader + 'a> {
    pub writer: &'a mut W,
    pub dir_reader: &'a R,
    path: &'a str,
    decorator: &'a Decorate<'a>
}


impl<'a, W: Write + 'a, R: DirReader + 'a> Les<'a, W, R> {

    pub fn new(
        path: &'a str,
        writer: &'a mut W,
        dir_reader: &'a R,
        decorator: &'a Decorate
    ) -> Les<'a, W, R> {

        Les { writer, dir_reader, path, decorator }
    }

    pub fn run(&mut self){
        let paths_result = self.dir_reader.read_dir(self.path);

        match paths_result {
            Ok(paths) => {

                for path in paths {

                    let paint_rules = self.decorator.get_paint_rules(&path);
                    println!("{}", paint(&paint_rules));
                }
            },
            _ => ()
        }

    }
}

#[cfg(test)]
mod tests {

    use les::*;
    use std;
    use fs;
    use decorate;

    #[test]
    fn it_doesnt_error() {

        let fs_reader = fs::FsReader;
        let mut writer = Writer;
        let decorator = decorate::Decorate::new(None);

        let mut l = Les::new("./", &mut writer, &fs_reader, &decorator);
        l.run();
    }


    struct Writer;

    impl std::io::Write for Writer {
        fn write(&mut self, _: &[u8]) -> Result<usize, std::io::Error>{ Ok(0) }
        fn flush(&mut self) -> Result<(), std::io::Error>{ Ok(()) }
    }


}
