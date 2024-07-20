use std::io::{BufRead, Error, Read};

pub struct Base64Reader<R: BufRead> {
    reader: R,
    small_buffer: [u8; 3],
    small_buffer_len: u8,
}

impl<R> Base64Reader<R>
where
    R: BufRead,
{
    pub fn new(reader: R) -> Base64Reader<R> {
        Base64Reader {
            reader,
            small_buffer: [0, 0, 0],
            small_buffer_len: 0u8,
        }
    }
}

impl<R> Read for Base64Reader<R>
where
    R: BufRead,
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let drained_small_buffer = self.drain_small_buffer(buf);
        if drained_small_buffer > 0usize {
            return Ok(drained_small_buffer);
        }
        let maybe_word = self.read_word()?;
        match maybe_word {
            Some(word) => {
                if buf.len() >= 4 {
                    let mut destination = &mut buf[0..4];
                    let count = decode_word(word, &mut destination)?;
                    Ok(count)
                } else {
                    let mut destination: [u8; 4] = [0, 0, 0, 0];
                    let count = decode_word(word, &mut destination)?;
                    if count == 0 {
                        return Ok(0);
                    }
                    for cursor in 0..buf.len() {
                        buf[cursor] = destination[cursor];
                    }
                    for cursor in buf.len()..count {
                        self.small_buffer[cursor - buf.len()] = destination[cursor];
                    }
                    self.small_buffer_len = (count - buf.len()) as u8;
                    Ok(buf.len())
                }
            }
            None => Ok(0),
        }
    }
}

impl<R> Base64Reader<R>
where
    R: BufRead,
{
    fn clear_small_buffer(&mut self) {
        self.small_buffer = [0u8; 3];
        self.small_buffer_len = 0;
    }

    fn drain_small_buffer(&mut self, buf: &mut [u8]) -> usize {
        if self.small_buffer_len == 0 {
            return 0;
        }
        for cursor_write in 0..buf.len() {
            if self.small_buffer_len as usize == cursor_write {
                self.clear_small_buffer();
                return cursor_write;
            }
            buf[cursor_write] = self.small_buffer[cursor_write];
        }
        match self.small_buffer_len - (buf.len() as u8) {
            0 => self.clear_small_buffer(),
            1 => {
                self.small_buffer_len = 1;
                self.small_buffer = [self.small_buffer[buf.len()], 0, 0]
            }
            2 => {
                self.small_buffer_len = 2;
                self.small_buffer = [self.small_buffer[1], self.small_buffer[2], 0];
            }
            _ => {
                panic!("small buffer has length of 3 maximum");
            }
        }
        buf.len()
    }

    fn read_word(&mut self) -> std::io::Result<Option<[u8; 4]>> {
        let mut result = [0u8; 4];
        let mut i = 0;
        while i <= 3 {
            let had_more = self.reader.read(&mut result[i..i + 1])? != 0;
            let had_more_alphabet = had_more && decode_ordinal(result[i]).is_some();
            let had_nothing = i == 0;
            match (had_more, had_more_alphabet, had_nothing) {
                (true, true, _) => {
                    i += 1;
                }
                (true, false, _) => {
                    //Try again
                }
                (false, true, _) => {
                    panic!("should not happen");
                }
                (false, false, true) => {
                    return Ok(None);
                }
                (false, false, false) => {
                    return Err(Error::new(
                        std::io::ErrorKind::UnexpectedEof,
                        "Unexpected EOF while decoding base64",
                    ));
                }
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
        [true, _, _, _] | [_, true, _, _] | [false, false, true, false] => Err(
            std::io::Error::new(std::io::ErrorKind::InvalidData, "invalid base64 padding"),
        ),
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
