// les module
use std::io::Write;
use fs::DirReader;
use style::Painter;

pub struct Les<'path, 'w, 'fs, 'paint, W: Write + 'w, R: DirReader + 'fs> {
    pub writer: &'w mut W,
    pub dir_reader: &'fs R,
    path: &'path str,
    painter: &'paint Painter
}


impl<'path, 'w, 'fs, 'paint, W: Write + 'w, R: DirReader + 'fs> Les<'path, 'w, 'fs,  'paint, W, R> {

    pub fn new(
        path: &'path str, 
        writer: &'w mut W, 
        dir_reader: &'fs R, 
        painter: &'paint Painter
    ) -> Les<'path, 'w, 'fs, 'paint, W, R> {

        Les { writer, dir_reader, path, painter }
    }

    pub fn run(&mut self){
        let paths_result = self.dir_reader.read_dir(self.path);

        match paths_result {
            Ok(paths) => {

                for path in paths {
                    let _ = writeln!(self.writer, "{}", self.painter.paint(path));
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

        let mut l = Les::new("./", &mut writer, &fs_reader, &Painter);
        l.run();
    }


    struct Writer;

    impl std::io::Write for Writer {
        fn write(&mut self, _: &[u8]) -> Result<usize, std::io::Error>{ Ok(0) }
        fn flush(&mut self) -> Result<(), std::io::Error>{ Ok(()) }
    }

}
