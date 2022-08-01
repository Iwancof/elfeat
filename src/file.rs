use std::fs::File;
use std::io::Read;

pub struct BinaryWrapper {
    binary: Vec<u8>,
    size: usize,
    current_pos: usize,
}

pub struct InterpretRef<'a, T> {
    offset: usize,
    inner: &'a mut T,
}

pub struct InterpretObj<T> {
    offset: usize,
    inner: T,
}

impl<'a, T> InterpretRef<'a, T> {
    pub fn inner(self) -> &'a mut T {
        self.inner
    }

    pub fn instantiate(self) -> InterpretObj<T>
    where
        T: Clone,
    {
        InterpretObj {
            offset: self.offset,
            inner: self.inner.clone(),
        }
    }
}

impl<T> AsRef<T> for InterpretObj<T> {
    fn as_ref(&self) -> &T {
        &self.inner
    }
}
impl<T> AsMut<T> for InterpretObj<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.inner
    }
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

    pub fn interpret_at<'a, T>(&'a mut self, at: usize) -> InterpretRef<'a, T> {
        let size = std::mem::size_of::<T>();

        let left = at;
        let right = left + size;

        let current_ptr = self.binary[left..right].as_mut_ptr();
        let ret_ref = unsafe { std::mem::transmute(current_ptr) };

        InterpretRef {
            offset: self.current_pos,
            inner: ret_ref,
        }
    }
    pub fn interpret_next<'a, T>(&'a mut self) -> InterpretRef<'a, T> {
        let size = std::mem::size_of::<T>();

        let at = self.current_pos;
        self.current_pos += size;

        self.interpret_at(at)
    }

    pub fn write_back_obj<T>(&mut self, obj: InterpretObj<T>) -> ()
    where
        T: Copy,
    {
        // is result need?
        let size = std::mem::size_of::<T>();
        let at = obj.offset;

        assert!(at + size <= self.size);

        let left = at;
        let right = left + size;

        let dest = self.binary[left..right].as_mut_ptr();
        let from = &obj.inner as *const _ as *const u8;

        unsafe {
            std::ptr::copy(from, dest, size);
        }
    }
}
