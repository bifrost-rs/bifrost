use std::io;

use bytes::{BufMut, BytesMut};

pub struct BytesMutWriter<'a>(&'a mut BytesMut);

impl<'a> BytesMutWriter<'a> {
    pub fn new(inner: &'a mut BytesMut) -> Self {
        Self(inner)
    }
}

impl<'a> io::Write for BytesMutWriter<'a> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.reserve(buf.len());
        self.0.put_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
