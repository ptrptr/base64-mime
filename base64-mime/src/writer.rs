use std::io::Write;

pub struct Base64Writer {}

impl Base64Writer {
    pub fn new(writer: &dyn Write) -> Base64Writer {
        Base64Writer {}
    }
}

impl Write for Base64Writer {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        todo!()
    }

    fn flush(&mut self) -> std::io::Result<()> {
        todo!()
    }
}
