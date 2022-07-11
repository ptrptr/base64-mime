use std::io::Write;

pub struct Base64Writer<W: Write> {
    writer: W,
    buffer: Vec<u8>,
}

impl<W> Base64Writer<W>
where
    W: Write,
{
    pub fn new(writer: W) -> Base64Writer<W> {
        Base64Writer {
            writer,
            buffer: Vec::new(),
        }
    }
}

impl<W> Write for Base64Writer<W>
where
    W: Write,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self.read_word(buf) {
            None => Ok(0),
            Some(word) => {
                let encoded_word = encode_word(&word);
                self.writer.write_all(&encoded_word[..])?;
                Ok(4)
            }
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self.buffer.len() {
            0 => Ok(()),
            3.. => panic!("should never happen"),
            len => {
                self.buffer.resize(3, 0);
                let word = encode_word(&self.buffer);
                let padded = pad(word, 3 - len);
                self.writer.write_all(&padded)?;
                Ok(())
            }
        }
    }
}

impl<W> Base64Writer<W>
where
    W: Write,
{
    fn read_word(&mut self, new_bytes: &[u8]) -> Option<[u8; 3]> {
        match (self.buffer.len(), new_bytes.len()) {
            (0, 0..=2) | (1, 1) => {
                self.buffer.extend_from_slice(new_bytes);
                None
            }
            (0, 3..) => Some(new_bytes[0..=2].try_into().expect("should never happen")),
            (1, 2..) => {
                let result = [self.buffer[0], new_bytes[0], new_bytes[1]];
                self.buffer.clear();
                Some(result)
            }
            (2, 1..) => {
                let result = [self.buffer[0], self.buffer[1], new_bytes[0]];
                self.buffer.clear();
                Some(result)
            }
            (_, _) => panic!("should never happen"),
        }
    }
}

fn pad(word: [u8; 4], amount: usize) -> [u8; 4] {
    match amount {
        0 => word,
        1 => [word[0], word[1], word[2], '=' as u8],
        2 => [word[0], word[1], '=' as u8, '=' as u8],
        _ => panic!("should never happen"),
    }
}

fn encode_word(buf: &[u8]) -> [u8; 4] {
    let mut ordinals = [0u8; 4];
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
