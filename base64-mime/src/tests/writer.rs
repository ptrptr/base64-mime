use std::io::Write;

use crate::Base64Writer;

#[test]
fn test_new_writer() {
    let mut empty: &mut [u8] = &mut [0u8; 0];
    let _writer: Base64Writer = Base64Writer::new(&mut empty);
}

#[test]
fn test_empty_write() -> std::io::Result<()> {
    let mut empty: &mut [u8] = &mut [0u8; 0];
    let mut writer: Base64Writer = Base64Writer::new(&mut empty);
    let _ = writer.write_all("".as_bytes())?;
    assert_eq!("".as_bytes(), empty, "writing empty should result in empty");
    Ok(())
}
