
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
        let name = format!("{}", stem.to_str().unwrap());
        let label = format!("{}", stem.to_str().unwrap());

        let path_buf = path.to_path_buf();
        let canonical_path = fs::canonicalize(&path_buf).unwrap_or(path_buf);
        let buf = canonical_path.to_str();
        let path_str = buf.unwrap_or(path.to_str().unwrap());

        File::new(
            name, label, meta.is_dir(), String::from(path_str)
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
    name: String,
    label: String,
    is_dir: bool,
    path: String
}

impl File {
    pub fn new(name: String, label: String, is_dir: bool, path: String) -> File {
        File {
            name,
            label,
            is_dir,
            path
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
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
