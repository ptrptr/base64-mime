use crate::Base64Reader;
use std::io::Read;

#[test]
fn test_new_reader() {
    let empty = String::new();
    let _reader = Base64Reader::new(empty.as_bytes());
}

#[test]
fn test_read_empty() -> std::io::Result<()> {
    template_read_test_with_text_and_expected("", "")
}

#[test]
fn test_read_only_nonalphabet() -> std::io::Result<()> {
    template_read_test_with_text_and_expected("\r\n", "")
}

#[test]
fn test_read_multiworld_with_nonalphabet_word_aligned() -> std::io::Result<()> {
    template_read_test_with_text_and_expected("Rm9v\r\nRm9v", "FooFoo")
}

#[test]
fn test_read_unpadded() -> std::io::Result<()> {
    template_read_test_with_text_and_expected("Rm9v", "Foo")
}

#[test]
fn test_read_one_padding_byte() -> std::io::Result<()> {
    template_read_test_with_text_and_expected("Rm9=", "Fo")
}

#[test]
fn test_read_two_padding_bytes() -> std::io::Result<()> {
    template_read_test_with_text_and_expected("Rm==", "F")
}

#[test]
fn test_read_multiword_without_padding() -> std::io::Result<()> {
    template_read_test_with_text_and_expected("SGVsbG8gd29ybGQh", "Hello world!")
}

#[test]
fn test_read_multiword_with_padding() -> std::io::Result<()> {
    template_read_test_with_text_and_expected("SGVsbG8gd29ybGQ=", "Hello world")
}

fn template_read_test_with_text_and_expected(
    text: &'static str,
    expected: &'static str,
) -> std::io::Result<()> {
    let mut reader = Base64Reader::new(text.as_bytes());
    let mut buf: Vec<u8> = Vec::new();
    let count = reader.read_to_end(&mut buf)?;
    assert_eq!(
        expected.len(),
        count,
        "Should read {} byte(s)",
        expected.len()
    );
    assert_eq!(
        expected.as_bytes(),
        buf,
        "\"{}\" should read as \"{}\"",
        text,
        expected
    );
    Ok(())
}
