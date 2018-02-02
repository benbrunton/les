
use std::fs;
use std::path::Path;

pub trait DirReader {

    fn read_dir(&self, path: &str) -> Result<Vec<File>, ReadError>;
}


pub struct FsReader;

impl FsReader{

    pub fn get_line(path: &Path) -> File {
        let meta = path.metadata().unwrap();
        let slash = if meta.is_dir() { "/" } else { "" };
        let stem_option = path.file_name();
        let stem = stem_option.unwrap();
        let label = format!("{}{}", stem.to_str().unwrap(), slash);
        let dir_type = if meta.is_dir() {
            DirType::Dir
        } else {
            DirType::File
        };

        File::new(
            label, dir_type
        )

    }

}

impl DirReader for FsReader {

    fn read_dir(&self, path: &str) -> Result<Vec<File>, ReadError> {
        let mut vec = vec!();

        let meta = fs::metadata(path); 

        match meta {
            Ok(ref m) if !m.is_dir() => {
                vec.push(FsReader::get_line(Path::new(path)));
                return Ok(vec);
            },
            Err(_) => return Err(ReadError::InvalidPath),
            _ => ()
        }

        let paths = fs::read_dir(path).unwrap();

        for p in paths {
            let dir_entry = p.unwrap();
            let file = FsReader::get_line(&dir_entry.path()); 
            vec.push(file)
        }

        Ok(vec)

    }

}

#[derive(Clone)]
pub enum DirType {
    File,
    Dir
}

pub struct File {
    label: String,
    dir_type: DirType
}

impl File {
    pub fn new(label: String, dir_type: DirType) -> File {
        File {
            label,
            dir_type
        }
    }

    pub fn get_dir_type(&self) -> DirType {
        self.dir_type.clone()
    }

    pub fn get_label(&self) -> String {
        self.label.clone()
    }

}

pub enum ReadError{
    InvalidPath
}
