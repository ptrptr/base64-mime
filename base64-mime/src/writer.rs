use std::io::Write;

pub struct Base64Writer {}

impl Base64Writer {
    pub fn new(writer: &dyn Write) -> Base64Writer {
        Base64Writer {}
    }
}
