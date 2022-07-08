use std::io::{Read, Write};

pub struct Base64Reader {}

pub struct Base64Writer {}

impl Base64Reader {
    pub fn new(reader: &dyn Read) -> Base64Reader {
        Base64Reader {}
    }
}

impl Base64Writer {
    pub fn new(writer: &dyn Write) -> Base64Writer {
        Base64Writer {}
    }
}

#[cfg(test)]
mod tests;
