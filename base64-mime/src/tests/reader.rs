use crate::Base64Reader;
use std::io::Read;

#[test]
fn test_new_reader() {
    let empty = String::new();
    let _reader = Base64Reader::new(empty.as_bytes());
}

#[test]
fn test_empty_read_to_end() -> std::io::Result<()> {
    let empty: String = String::new();
    let mut reader = Base64Reader::new(empty.as_bytes());
    let mut buf: Vec<u8> = Vec::new();
    reader.read_to_end(&mut buf)?;
    assert_eq!("".as_bytes(), buf, "empty should read as empty");
    Ok(())
}

#[test]
fn test_read_unpadded() -> std::io::Result<()> {
    let mut reader = Base64Reader::new("Rm9v".as_bytes());
    let mut buf: Vec<u8> = Vec::new();
    let count = reader.read_to_end(&mut buf)?;
    assert_eq!(3, count, "Should read 3 bytes");
    assert_eq!("Foo".as_bytes(), buf, "\"Rm9v\" should read as \"Foo\"");
    Ok(())
}

#[test]
fn test_read_one_padding_byte() -> std::io::Result<()> {
    let mut reader = Base64Reader::new("Rm9=".as_bytes());
    let mut buf: Vec<u8> = Vec::new();
    let count = reader.read_to_end(&mut buf)?;
    assert_eq!(2, count, "Should read 2 bytes");
    assert_eq!("Fo".as_bytes(), buf, "\"Rm9=\" should read as \"Fo\"");
    Ok(())
}
