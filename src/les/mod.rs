// les module
use std::io::Write;
use fs::DirReader;
use decorate::Decorate;
use paintitems::PaintItems;
use io::Print;

pub struct Les<'a, P: Print + 'a, R: DirReader + 'a> {
    pub printer: &'a P,
    pub dir_reader: &'a R,
    path: &'a str,
}


impl<'a, P: Print + 'a, R: DirReader + 'a> Les<'a, P, R> {

    pub fn new(
        path: &'a str,
        printer: &'a P,
        dir_reader: &'a R
    ) -> Les<'a, P, R> {

        Les { printer, dir_reader, path }
    }

    pub fn run(&mut self){
        let paths_result = self.dir_reader.read_dir(self.path);

        match paths_result {
            Ok(paths) => {
                self.printer.print(paths);
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
    use io;

    #[test]
    fn it_doesnt_error() {

        let fs_reader = fs::FsReader;
        let printer = Printer;
        let decorator = decorate::Decorate::new(None);

        let mut l = Les::new("./", &printer, &fs_reader);
        l.run();
    }


    struct Printer;

    impl io::Print for Printer {
        fn print(&self, _: Vec<fs::File>) { }
    }


}
