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
                let count = decode_word(word, &mut destination)?;
                Ok(count)
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
            let had_more = self.reader.read(&mut result[i..i + 1])? != 0
                && decode_ordinal(result[i]).is_some();
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

fn decode_word(word: [u8; 4], destination: &mut [u8]) -> std::io::Result<usize> {
    let symbol_count = padding_check(word)?;
    let ordinals = word.map(|a| decode_ordinal(a).expect("should not happen"));
    destination[0] = (ordinals[0] << 2) | (ordinals[1] >> 4);
    if symbol_count == 1 {
        return Ok(1);
    }
    destination[1] = (ordinals[1] << 4) | (ordinals[2] >> 2);
    if symbol_count == 2 {
        return Ok(2);
    }
    destination[2] = (ordinals[2] << 6) | ordinals[3];
    Ok(3)
}

fn padding_check(word: [u8; 4]) -> std::io::Result<u8> {
    let padding: [bool; 4] = word.map(|x| x == '=' as u8);
    match padding {
        [true, _, _, _] | [_, true, _, _] | [false, false, true, false] => {
            todo!("handle bad padding")
        }
        [false, false, false, false] => Ok(3),
        [false, false, false, true] => Ok(2),
        [false, false, true, true] => Ok(1),
    }
}

fn decode_ordinal(ordinal: u8) -> Option<u8> {
    match ordinal as char {
        'A'..='Z' => Some(ordinal - 'A' as u8),
        'a'..='z' => Some(26 + (ordinal - 'a' as u8)),
        '0'..='9' => Some(52 + (ordinal - '0' as u8)),
        '+' => Some(62),
        '/' => Some(63),
        '=' => Some(0),
        _ => None,
    }
}
