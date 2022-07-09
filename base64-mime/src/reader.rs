use std::io::{BufRead, Read};

pub struct Base64Reader<R: BufRead> {
    reader: R,
}

impl<R> Base64Reader<R>
where
    R: BufRead,
{
    pub fn new(reader: R) -> Base64Reader<R> {
        Base64Reader { reader }
    }
}

impl<R> Read for Base64Reader<R>
where
    R: BufRead,
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let maybe_word = self.read_word()?;
        match maybe_word {
            Some(word) => {
                let mut destination = &mut buf[0..4];
                decode_word(word, &mut destination);
                Ok(3)
            }
            None => Ok(0),
        }
    }
}

impl<R> Base64Reader<R>
where
    R: BufRead,
{
    fn read_word(&mut self) -> std::io::Result<Option<[u8; 4]>> {
        let mut result = [0u8; 4];
        for i in 0..=3 {
            let had_more = self.reader.read(&mut result[i..i + 1])? != 0;
            if had_more {
                continue;
            }
            if i == 0 {
                return Ok(None);
            } else {
                todo!("Handle EOF within word boundary")
            }
        }
        Ok(Some(result))
    }
}

fn decode_word(word: [u8; 4], destination: &mut [u8]) {
    let ordinals = word.map(decode_ordinal);
    destination[0] = (ordinals[0] << 2) | (ordinals[1] >> 4);
    destination[1] = (ordinals[1] << 4) | (ordinals[2] >> 2);
    destination[2] = (ordinals[2] << 6) | ordinals[3];
}

fn decode_ordinal(ordinal: u8) -> u8 {
    match ordinal as char {
        'A'..='Z' => ordinal - 'A' as u8,
        'a'..='z' => 26 + (ordinal - 'a' as u8),
        '0'..='9' => 52 + (ordinal - '0' as u8),
        '+' => 62,
        '/' => 63,
        _ => todo!("Handle bad characters"),
    }
}
