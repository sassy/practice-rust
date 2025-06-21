use std::io::Read;
use thiserror::Error;

#[derive(Debug, Error)]
enum MyError {
    #[error("file not open")]
    FileOpenError(#[from] std::io::Error),
    #[error("file read error: {0}")]
    FileReadError(String),
}

fn read_file(path: &str) -> Result<String, MyError> {
    let mut file = std::fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|_| MyError::FileReadError("Failed to read file".to_owned()))?;
    Ok(contents)
}

fn main() {
    let ret = read_file("../README.md");
    match ret {
        Ok(c) => println!("{:?}", c),
        Err(e) => println!("Error reading file: {}", e),
    }
    let ret2 = read_file("foo.txt");
    match ret2 {
        Ok(c) => println!("{:?}", c),
        Err(e) => println!("Error reading file: {}", e),
    }
}
