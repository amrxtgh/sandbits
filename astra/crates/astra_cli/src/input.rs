use std::{fs, io};

// just file handling
pub fn read_source(filename: &str) -> Result<String, io::Error> {
    fs::read_to_string(filename)
}
