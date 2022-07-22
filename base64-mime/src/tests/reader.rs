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
fn test_read_multiword_with_nonalphabet_word_aligned() -> std::io::Result<()> {
    template_read_test_with_text_and_expected("Rm9v\r\nRm9v", "FooFoo")
}

#[test]
fn test_read_multiword_with_nonalphabet_splitting_word() -> std::io::Result<()> {
    template_read_test_with_text_and_expected("Rm9\r\nvRm9v", "FooFoo")
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

#[test]
fn test_bad_padding_pattern_0010() -> std::io::Result<()> {
    template_bad_padding_test_with_pattern([false, false, true, false])
}

#[test]
fn test_bad_padding_pattern_0100() -> std::io::Result<()> {
    template_bad_padding_test_with_pattern([false, true, false, false])
}

#[test]
fn test_bad_padding_pattern_1000() -> std::io::Result<()> {
    template_bad_padding_test_with_pattern([true, false, false, false])
}

#[test]
fn test_bad_padding_pattern_0110() -> std::io::Result<()> {
    template_bad_padding_test_with_pattern([false, true, true, false])
}

#[test]
fn test_bad_padding_pattern_1110() -> std::io::Result<()> {
    template_bad_padding_test_with_pattern([true, true, true, false])
}

#[test]
fn test_bad_padding_pattern_1111() -> std::io::Result<()> {
    template_bad_padding_test_with_pattern([true, true, true, true])
}

fn template_bad_padding_test_with_pattern(pattern: [bool; 4]) -> std::io::Result<()> {
    let mut base_text = Vec::from("Rm9v".as_bytes());
    for (index, value) in pattern.into_iter().enumerate() {
        if value {
            base_text[index] = '=' as u8;
        }
    }
    template_error_test_with_bytes_errorkind_and_contained_text(
        &base_text[..],
        std::io::ErrorKind::InvalidData,
        "padding",
    )
}

#[test]
fn test_unexpected_eof_length_one() -> std::io::Result<()> {
    template_error_test_with_bytes_errorkind_and_contained_text(
        "F".as_bytes(),
        std::io::ErrorKind::UnexpectedEof,
        "EOF",
    )
}

#[test]
fn test_unexpected_eof_length_two() -> std::io::Result<()> {
    template_error_test_with_bytes_errorkind_and_contained_text(
        "Fm".as_bytes(),
        std::io::ErrorKind::UnexpectedEof,
        "EOF",
    )
}

#[test]
fn test_unexpected_eof_length_three() -> std::io::Result<()> {
    template_error_test_with_bytes_errorkind_and_contained_text(
        "Fm9".as_bytes(),
        std::io::ErrorKind::UnexpectedEof,
        "EOF",
    )
}

fn template_error_test_with_bytes_errorkind_and_contained_text(
    bytes: &[u8],
    errorkind: std::io::ErrorKind,
    contained_text: &'static str,
) -> std::io::Result<()> {
    let mut reader = Base64Reader::new(bytes);
    let mut buf: Vec<u8> = Vec::new();
    let err = reader.read_to_end(&mut buf).expect_err("Expected error");
    assert!(
        err.kind().eq(&errorkind),
        "Error should be of kind {}",
        errorkind.to_string()
    );
    assert!(
        err.to_string().contains(contained_text),
        "Error description should contain \"{}\"",
        contained_text
    );
    Ok(())
}
