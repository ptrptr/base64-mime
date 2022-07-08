use std::io::Read;

pub struct Base64Reader {}

impl Base64Reader {
    pub fn new(reader: &dyn Read) -> Base64Reader {
        Base64Reader {}
    }
}
