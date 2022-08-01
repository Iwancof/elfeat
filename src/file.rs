use std::fs::File;
use std::io::Read;

pub struct BinaryWrapper {
    binary: Vec<u8>,
    size: usize,
    current_pos: usize,
}

impl BinaryWrapper {
    pub fn from_file(mut file: File) -> Self {
        let mut content = vec![];
        let size = file.read_to_end(&mut content).unwrap();

        Self {
            binary: content,
            size,
            current_pos: 0,
        }
    }
    pub fn from_vec(v: Vec<u8>) -> Self {
        Self {
            size: v.len(),
            binary: v,
            current_pos: 0,
        }
    }
    pub fn eat<'a, T>(&'a mut self) -> &'a mut T {
        let size = std::mem::size_of::<T>();

        println!("next: {}", size);

        let left = self.current_pos;
        let right = left + size;

        let current_ptr = self.binary[left..right].as_ptr();
        let ret_ref = unsafe { std::mem::transmute(current_ptr) };

        self.current_pos += size;

        ret_ref
    }
}
