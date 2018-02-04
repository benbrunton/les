// les module
use std::io::Write;
use fs::DirReader;
use style::Painter;

pub struct Les<'a, W: Write + 'a, R: DirReader + 'a> {
    pub writer: &'a mut W,
    pub dir_reader: &'a R,
    path: &'a str,
    painter: &'a Painter
}


impl<'a, W: Write + 'a, R: DirReader + 'a> Les<'a, W, R> {

    pub fn new(
        path: &'a str, 
        writer: &'a mut W, 
        dir_reader: &'a R, 
        painter: &'a Painter
    ) -> Les<'a, W, R> {

        Les { writer, dir_reader, path, painter }
    }

    pub fn run(&mut self){
        let paths_result = self.dir_reader.read_dir(self.path);

        match paths_result {
            Ok(paths) => {

                for path in paths {
                    let output = self.painter.paint(path);
                    if let Some(painted_path) = output {
                        let _ = writeln!(self.writer, "{}", painted_path);
                    }
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
    use style::Painter;

    #[test]
    fn it_doesnt_error() {

        let fs_reader = fs::FsReader;
        let mut writer = Writer;
        let painter = Painter::new(None);

        let mut l = Les::new("./", &mut writer, &fs_reader, &painter);
        l.run();
    }


    struct Writer;

    impl std::io::Write for Writer {
        fn write(&mut self, _: &[u8]) -> Result<usize, std::io::Error>{ Ok(0) }
        fn flush(&mut self) -> Result<(), std::io::Error>{ Ok(()) }
    }

}
