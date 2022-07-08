use crate::{Base64Reader, Base64Writer};

#[test]
fn test_new_reader() {
    let empty = String::new();
    let _reader = Base64Reader::new(&empty.as_bytes());
}

#[test]
fn test_new_writer() {
    let mut empty: &mut [u8] = &mut [0u8; 0];
    let _writer: Base64Writer = Base64Writer::new(&mut empty);
}
