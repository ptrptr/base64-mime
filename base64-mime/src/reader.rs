use std::io::Read;

pub struct Base64Reader {}

impl Base64Reader {
    pub fn new(reader: &dyn Read) -> Base64Reader {
        Base64Reader {}
    }
}

impl Read for Base64Reader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        Ok(0)
    }
}
