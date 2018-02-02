use std::fs;

pub trait DirReader {

    fn read_dir(&self, path: &str) -> Vec<File>;
}


pub struct FsReader;

impl DirReader for FsReader {

    fn read_dir(&self, path: &str) -> Vec<File> {
       let paths = fs::read_dir(path).unwrap();
       let mut vec = vec!();

        for path in paths {
            let dir_entry = path.unwrap();
            let meta = dir_entry.metadata().unwrap();
            let slash = if meta.is_dir() { "/" } else { "" };
            let dir_path = dir_entry.path();
            let stem_option = dir_path.file_name();
            let stem = stem_option.unwrap();
            let label = format!("{}{}", stem.to_str().unwrap(), slash);
            let dir_type = if meta.is_dir() {
                DirType::Dir
            } else {
                DirType::File
            };

            let f = File {
                label, dir_type
            };

            vec.push(f)
        }

        vec

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

    pub fn to_str(&self) -> String {
        self.label.clone()
    }
}
