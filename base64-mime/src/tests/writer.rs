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
fn test_write_zero_bytes_return_value() -> std::io::Result<()> {
    template_write_call_return_value_test("")
}

#[test]
fn test_write_one_byte_return_value() -> std::io::Result<()> {
    template_write_call_return_value_test("F")
}

#[test]
fn test_write_two_bytes_return_value() -> std::io::Result<()> {
    template_write_call_return_value_test("Fo")
}

#[test]
fn test_write_three_bytes_return_value() -> std::io::Result<()> {
    template_write_call_return_value_test("Foo")
}

#[test]
fn test_empty_write() -> std::io::Result<()> {
    template_write_test_with_text_and_expected("", "")
}

#[test]
fn test_write_unpadded() -> std::io::Result<()> {
    template_write_test_with_text_and_expected("Foo", "Rm9v")
}

#[test]
fn test_write_one_padding_byte() -> std::io::Result<()> {
    template_write_test_with_text_and_expected("Fo", "Rm8=")
}

#[test]
fn test_write_two_padding_bytes() -> std::io::Result<()> {
    template_write_test_with_text_and_expected("F", "Rg==")
}

fn template_write_call_return_value_test(text: &'static str) -> std::io::Result<()> {
    let mut buffer: Vec<u8> = Vec::new();
    let mut writer = Base64Writer::new(&mut buffer);
    let count = writer.write(text.as_bytes())?;
    assert_eq!(
        text.len(),
        count,
        "expected write call to return {}, but got {}",
        text.len(),
        count,
    );
    Ok(())
}

fn template_write_test_with_text_and_expected(
    text: &'static str,
    expected: &'static str,
) -> std::io::Result<()> {
    let mut buffer: Vec<u8> = Vec::new();
    let mut writer = Base64Writer::new(&mut buffer);
    let _ = writer.write(text.as_bytes())?;
    writer.flush()?;
    assert_eq!(
        expected.len(),
        buffer.len(),
        "{} input bytes should produce {} output bytes",
        text.len(),
        expected.len(),
    );
    assert_eq!(
        expected.as_bytes(),
        buffer,
        "\"{}\" should encode to \"{}\"",
        text,
        expected
    );
    Ok(())
}
