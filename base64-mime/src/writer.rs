use std::io::Write;

pub struct Base64Writer<W : Write> {
    writer: W
}

impl<W> Base64Writer<W> where W : Write {
    pub fn new(writer: W) -> Base64Writer<W> {
        Base64Writer {writer}
    }
}

impl<W> Write for Base64Writer<W> where W : Write {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match buf.len() {
            0 =>    Ok(0),
            1..=2 => todo!("handle small buffer"),
            _ => {
                let word = &buf[0..=2];
                let encoded_word = encode_word(&word);
                self.writer.write_all(&encoded_word[..])?;
                Ok(4)
            },
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        todo!()
    }
}

fn encode_word(buf: &[u8]) -> [u8;4] {
    let mut ordinals = [0u8;4];
    ordinals[0] = buf[0] >> 2;
    ordinals[1] = ((buf[0] << 4) | (buf[1] >> 4)) & 0b00111111u8;
    ordinals[2] = ((buf[1] << 2) | (buf[2] >> 6)) & 0b00111111u8;
    ordinals[3] = buf[2] & 0b00111111u8;
    ordinals.map(encode_symbol)
}

fn encode_symbol(ordinal: u8) -> u8 {
    match ordinal {
        0..=25 => 'A' as u8 + ordinal,
        26..=51 => 'a' as u8 + (ordinal - 26),
        52..=61 => '0' as u8 + (ordinal - 52),
        62 => '+' as u8, 
        63 => '/' as u8,
        _ => panic!("should never happen"),
    }
}