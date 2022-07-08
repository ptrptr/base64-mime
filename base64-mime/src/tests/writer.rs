use crate::Base64Writer;

#[test]
fn test_new_writer() {
    let mut empty: &mut [u8] = &mut [0u8; 0];
    let _writer: Base64Writer = Base64Writer::new(&mut empty);
}
