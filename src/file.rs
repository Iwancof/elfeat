use std::fs::File;
use std::io::Read;

trait BinaryReader {
    fn read(&self, start: usize, len: usize) -> &[u8];
    fn write(&mut self, start: usize, data: &[u8]);
}

struct Sequential {
    vector: Vec<u8>,
}

impl BinaryReader for Sequential {
    fn read(&self, start: usize, len: usize) -> &[u8] {
        &self.vector[start..start + len]
    }
    fn write(&mut self, start: usize, data: &[u8]) {
        let length = data.len();
        self.vector[start..start + length].copy_from_slice(data);
    }
}

struct Seekable<'a, T>
where
    T: BinaryReader,
{
    inner: &'a T,
    pos: usize,
}
