// les module
use std::io::Write;
use fs::DirReader;

pub struct Les</*'args,*/ 'w, 'fs, W: Write + 'w, R: DirReader + 'fs> {

    pub writer: &'w mut W,

//    pub args: Vec<&'args OsStr>

    pub dir_reader: &'fs R
}


impl</*'args,*/ 'w, 'fs, W: Write + 'w, R: DirReader + 'fs> Les</*'args,*/ 'w, 'fs,  W, R> {

    pub fn new(writer: &'w mut W, dir_reader: &'fs R) -> Les<'w, 'fs, W, R> {

        Les { writer, dir_reader }
    }

    pub fn run(&mut self){
        let paths = self.dir_reader.read_dir("./");

        for path in paths {
            let _ = writeln!(self.writer, "{}", path.to_str());
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
        let mut writer = std::io::stdout();

        let mut l = Les::new(&mut writer, &fs_reader);
        l.run();
    }

}
