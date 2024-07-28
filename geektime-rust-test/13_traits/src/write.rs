use std::{fmt::Debug, io::Write};

pub struct BufBuilder {
    buf: Vec<u8>,
}

impl BufBuilder {
    fn new() -> BufBuilder {
        Self {
            buf: Vec::with_capacity(1024),
        }
    }
}

impl Debug for BufBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(self.buf.as_ref()))
    }
}

impl Write for BufBuilder {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buf.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

fn main() {
    let mut buf_builder = BufBuilder::new();
    let buf1 = b"hello world";
    buf_builder.write_all(buf1).unwrap();
    println!("{:?}", buf_builder)
}

// write!怎么用
