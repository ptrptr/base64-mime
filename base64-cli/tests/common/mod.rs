use std::{
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};

/**
 * Functions for integration tests
 */

// Get file path
pub fn get_file(name: &'static str) -> PathBuf {
    return Path::new(env!("CARGO_TARGET_TMPDIR")).join(name);
}

pub fn write_file(name: &'static str, content: &[u8]) -> std::io::Result<()> {
    let mut file = File::create(get_file(name))?;
    file.write_all(content)?;
    Ok(())
}

pub fn read_file(name: &'static str) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(get_file(name))?;
    let mut result: Vec<u8> = Vec::new();
    file.read_to_end(&mut result)?;
    Ok(result)
}
