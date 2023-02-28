use std::io::{self, Read, Write, Seek, SeekFrom};

pub struct StreamBuffer {
    pos: usize,
    len: usize,
    buf: Vec<u8>,
}

impl StreamBuffer {
    pub fn new(size: usize) -> Self {
        Self {
            pos: 0,
            len: 0,
            buf: vec![0u8; size],
        }
    }

    pub fn from(buf: Vec<u8>) -> Self {
        let len = buf.len();
        Self { pos: 0, len, buf }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.buf[..self.len].to_vec()
    }

    pub fn to_vec_from_pos(&self) -> Vec<u8> {
        let num = self.len - self.pos;
        if num <= 0 {
            return vec![];
        }
        self.buf[self.pos..self.len].to_vec()
    }

    pub fn compact(&mut self) {
        let num = self.len - self.pos;
        if num > 0 {
            self.buf.copy_within(self.pos..self.len, 0);
        }
        self.len = num;
        self.pos = 0;
    }

    pub fn get_ref(&self) -> &[u8] {
        &self.buf[..self.len]
    }

    pub fn get_ref_and_advance(&mut self, length: usize) -> &[u8] {
        let start = self.pos;
        self.pos += length;
        &self.buf[start..self.pos]
    }

    fn check_size(&mut self, size: usize) {
        if size <= self.buf.len() {
            return;
        }
        let mut num = self.buf.len().max(1);
        while size > num {
            num *= 2;
        }
        self.buf.resize(num, 0);
    }
}

impl Default for StreamBuffer {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Read for StreamBuffer {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let num = self.len - self.pos;
        if num <= 0 {
            return Ok(0);
        }
        let count = num.min(buf.len());
        buf[..count].copy_from_slice(&self.buf[self.pos..(self.pos+count)]);
        self.pos += count;
        Ok(count)
    }
}

impl Write for StreamBuffer {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let num = self.pos + buf.len();
        self.check_size(num);
        if num > self.len {
            self.len = num;
        }
        self.buf[self.pos..num].copy_from_slice(buf);
        self.pos = num;
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl Seek for StreamBuffer {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        match pos {
            SeekFrom::Start(pos) => self.pos = pos as usize,
            SeekFrom::End(pos) => self.pos = self.len - pos as usize,
            SeekFrom::Current(pos) => self.pos = self.pos + pos as usize,
        }
        Ok(self.pos as u64)
    }
}


impl StreamBuffer {
    pub fn read_byte(&mut self) -> io::Result<u8> {
        if self.pos >= self.len {
            return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                "StreamBuffer.ReadByte() failed.",
            ));
        }
        let b = self.buf[self.pos];
        self.pos += 1;
        Ok(b)
    }

    pub fn write_byte(&mut self, value: u8) -> io::Result<()> {
        let num = self.pos + 1;
        self.check_size(num);
        if num > self.len {
            self.len = num;
        }
        self.buf[self.pos] = value;
        self.pos = num;
        Ok(())
    }

    pub fn write_bytes(&mut self, values: &[u8]) -> io::Result<()> {
        let num = self.pos + values.len();
        self.check_size(num);
        if num > self.len {
            self.len = num;
        }
        self.buf[self.pos..num].copy_from_slice(values);
        self.pos = num;
        Ok(())
    }

    fn position(&self) -> u64 {
        self.pos as u64
    }
}

impl std::fmt::Debug for StreamBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StreamBuffer")
            .field("pos", &self.pos)
            .field("len", &self.len)
            .field("buf", &self.to_vec())
            .finish()
    }
}