use std::fs::{File, OpenOptions};
use std::io::Error;
use std::path::Path;

pub fn create_db_file(path: &str) -> Result<File, Error> {
    if Path::new(path).exists() {
        panic!("Filepath already exists");
    }

    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
}

pub fn open_db_file(path: &str) -> Result<File, Error> {
    OpenOptions::new().read(true).write(true).open(path)
}
