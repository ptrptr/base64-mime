use crate::Base64Reader;

#[test]
fn test_new_reader() {
    let empty = String::new();
    let _reader = Base64Reader::new(&empty.as_bytes());
}
