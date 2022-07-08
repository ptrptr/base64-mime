use crate::Base64Reader;
use std::io::Read;

#[test]
fn test_new_reader() {
    let empty = String::new();
    let _reader = Base64Reader::new(&empty.as_bytes());
}

#[test]
fn test_empty_read_to_end() -> std::io::Result<()> {
    let empty: String = String::new();
    let mut reader = Base64Reader::new(&empty.as_bytes());
    let mut buf: Vec<u8> = Vec::new();
    reader.read_to_end(&mut buf)?;
    assert_eq!("".as_bytes(), buf, "empty should read as empty");
    Ok(())
}
