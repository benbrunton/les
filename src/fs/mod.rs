
use std::fs;
use std::path::Path;

pub trait DirReader {

    fn read_dir(&self, path: &str) -> Result<Vec<File>, ReadError>;
}


pub struct FsReader;

impl FsReader{

    pub fn get_line(path: &Path) -> Option<File> {
        let meta_result = path.metadata();
        match meta_result {
            Ok(meta) => Some(FsReader::get_file(path, meta)),
            _ => None
        }

    }

    pub fn get_file(path: &Path, meta: fs::Metadata) -> File {
        let stem_option = path.file_name();
        let stem = stem_option.unwrap();
        let label = format!("{}", stem.to_str().unwrap());

        File::new(
            label, meta.is_dir(), String::from(path.to_str().unwrap())
        )

    }

}

impl DirReader for FsReader {

    fn read_dir(&self, path: &str) -> Result<Vec<File>, ReadError> {
        let mut vec = vec!();

        let meta = fs::metadata(path); 

        match meta {
            Ok(ref m) if !m.is_dir() => {
                let file_result = FsReader::get_line(Path::new(path));
                match file_result {
                    Some(file) => {
                        vec.push(file);
                        return Ok(vec);

                    },
                    _ => return Err(ReadError::InvalidPath)
                }

            },
            Err(_) => return Err(ReadError::InvalidPath),
            _ => ()
        }

        let paths = fs::read_dir(path).unwrap();

        for p in paths {
            let dir_entry = p.unwrap();
            let file_result = FsReader::get_line(&dir_entry.path()); 
            match file_result {
                Some(file) => {
                    vec.push(file);

                },
                _ => ()
            }
        }

        Ok(vec)

    }

}


pub struct File {
    label: String,
    is_dir: bool,
    path: String
}

impl File {
    pub fn new(label: String, is_dir: bool, path: String) -> File {
        File {
            label,
            is_dir,
            path
        }
    }

    pub fn get_is_dir(&self) -> bool {
        self.is_dir
    }

    pub fn get_label(&self) -> String {
        self.label.clone()
    }

    pub fn get_path(&self) -> String {
        self.path.clone()
    }

}

pub enum ReadError{
    InvalidPath
}
