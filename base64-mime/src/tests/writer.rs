use std::io::Write;

use crate::Base64Writer;

#[test]
fn test_new_writer() {
    let empty: Vec<u8> = Vec::new();
    let _writer = Base64Writer::new(empty);
}

#[test]
fn test_empty_write_all() -> std::io::Result<()> {
    let mut empty: Vec<u8> = Vec::new();
    let mut writer = Base64Writer::new(&mut empty);
    let _ = writer.write_all("".as_bytes())?;
    assert_eq!("".as_bytes(), empty, "writing empty should result in empty");
    Ok(())
}

#[test]
fn test_empty_write() -> std::io::Result<()> {
    let mut empty: &mut [u8] = &mut [0u8; 0];
    let mut writer = Base64Writer::new(&mut empty);
    let _ = writer.write("".as_bytes())?;
    assert_eq!("".as_bytes(), empty, "writing empty should result in empty");
    Ok(())
}

#[test]
fn test_write_unpadded() -> std::io::Result<()> {
    let mut buffer: Vec<u8> = Vec::new();
    let mut writer = Base64Writer::new(&mut buffer);
    let count = writer.write("Foo".as_bytes())?;
    assert_eq!(4, count, "3 input bytes should produce 4 output bytes");
    assert_eq!("Rm9v".as_bytes(), buffer, "\"Foo\" should encode to \"Rm9v\"");
    Ok(())
}