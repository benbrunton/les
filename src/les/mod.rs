// les module
use std::io::Write;
use fs::DirReader;
use style::paint;

pub struct Les<'path, 'w, 'fs, W: Write + 'w, R: DirReader + 'fs> {

    pub writer: &'w mut W,

//    pub args: Vec<&'args OsStr>

    pub dir_reader: &'fs R,

    path: &'path str
}


impl<'path, 'w, 'fs, W: Write + 'w, R: DirReader + 'fs> Les<'path, 'w, 'fs,  W, R> {

    pub fn new(path: &'path str, writer: &'w mut W, dir_reader: &'fs R) -> Les<'path, 'w, 'fs, W, R> {

        Les { writer, dir_reader, path }
    }

    pub fn run(&mut self){
        let paths_result = self.dir_reader.read_dir(self.path);

        match paths_result {
            Ok(paths) => {

                for path in paths {
                    let _ = writeln!(self.writer, "{}", paint(path));
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

    #[test]
    fn it_doesnt_error() {

        let fs_reader = fs::FsReader;
        let mut writer = Writer;

        let mut l = Les::new("./", &mut writer, &fs_reader);
        l.run();
    }


    struct Writer;

    impl std::io::Write for Writer {
        fn write(&mut self, _: &[u8]) -> Result<usize, std::io::Error>{ Ok(0) }
        fn flush(&mut self) -> Result<(), std::io::Error>{ Ok(()) }
    }

}
