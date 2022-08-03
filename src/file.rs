use std::fs::File;
use std::io::Read;

use crate::types::repr_u8::{VOConstitudeResultMut, VOWrapU8ArrayMut};

#[derive(Debug, PartialEq, Eq)]
pub struct BinaryWrapper {
    binary: Vec<u8>,
}
#[derive(Debug, PartialEq, Eq)]
pub struct InterpretMut<'a, T> {
    offset: usize,
    inner: &'a mut T,
}
#[derive(Debug, PartialEq, Eq)]
pub struct InterpretRef<'a, T> {
    offset: usize,
    inner: &'a T,
}
#[derive(Debug, PartialEq, Eq)]
pub struct InterpretObj<T> {
    offset: usize,
    inner: T,
}

#[derive(Debug, PartialEq, Eq)]
pub enum InterpretErr {
    NotEnoughLength,
    InvalidRequiredRangeBound,
    InvalidSplit,
}

#[derive(Debug, PartialEq, Eq)]
pub enum InterpretResultMut<'a, T> {
    Valid(BinarySliceMut<'a>, InterpretMut<'a, T>, BinarySliceMut<'a>),
    Invalid(BinarySliceMut<'a>, InterpretMut<'a, T>, BinarySliceMut<'a>),
    Error(InterpretErr),
}

impl<'a, T> InterpretResultMut<'a, T> {
    pub fn ignore_validity(
        self,
    ) -> Option<(BinarySliceMut<'a>, InterpretMut<'a, T>, BinarySliceMut<'a>)> {
        match self {
            Self::Valid(head, body, remain) => Some((head, body, remain)),
            Self::Invalid(head, body, remain) => Some((head, body, remain)),
            Self::Error(_) => None,
        }
    }
    pub fn unwrap_valid(self) -> (BinarySliceMut<'a>, InterpretMut<'a, T>, BinarySliceMut<'a>)
    where
        T: core::fmt::Debug, // TODO: Fix
    {
        if let Self::Valid(h, i, s) = self {
            return (h, i, s);
        }

        panic!("Expect valid value. but found {:?}", self);
    }
    pub fn unwrap_no_head_validity(self) -> (InterpretMut<'a, T>, BinarySliceMut<'a>)
    where
        T: core::fmt::Debug,
    {
        let (head, body, tail) = self.ignore_validity().unwrap();
        assert_eq!(head.len(), 0);

        (body, tail)
    }
}

#[derive(Debug, PartialEq, Eq)] // No Clonable
pub struct BinarySliceMut<'a> {
    slice: &'a mut [u8],
    absolute_offset: usize,
}

impl<'a> BinarySliceMut<'a> {
    fn len(&'a self) -> usize {
        self.slice.len()
    }
    fn new(slice: &'a mut [u8], absolute_offset: usize) -> Self {
        Self {
            slice,
            absolute_offset,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BinarySlice<'a> {
    slice: &'a [u8],
    absolute_offset: usize,
}

impl<'a, T> InterpretRef<'a, T> {
    pub fn inner(self) -> &'a T {
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
        let _ = file.read_to_end(&mut content).unwrap();

        Self { binary: content }
    }
    pub fn from_vec(binary: Vec<u8>) -> Self {
        Self { binary }
    }

    pub fn to_mut_slice(&mut self) -> BinarySliceMut<'_> {
        BinarySliceMut::new(&mut self.binary, 0)
    }
}

impl<'a> BinarySliceMut<'a> {
    #[allow(unused)]
    fn split_at_mut(&'a mut self, at: usize) -> Option<(BinarySliceMut<'a>, BinarySliceMut<'a>)> {
        if self.slice.len() <= at {
            return None;
        }

        let (left, right) = self.slice.split_at_mut(at);

        let left = Self {
            slice: left,
            absolute_offset: self.absolute_offset,
        };

        let right = Self {
            slice: right,
            absolute_offset: self.absolute_offset + at,
        };

        Some((left, right))
    }

    /// split into 3 chunks.
    /// +-----
    /// |0 1 2 3 4 5 6 7 8 9 |
    ///      ^at1=2  ^at2=6  
    /// result: ([0, 1], [2, 3, 4, 5], [6, 7, 8, 9])
    ///
    fn split_at_2_mut(
        &'a mut self,
        at1: usize,
        at2: usize,
    ) -> Result<(BinarySliceMut<'a>, BinarySliceMut<'a>, BinarySliceMut<'a>), InterpretErr> {
        if !(at1 <= at2) {
            return Err(InterpretErr::InvalidSplit);
        }

        if !(at2 < self.slice.len()) {
            return Err(InterpretErr::InvalidRequiredRangeBound);
        }

        let head_offset = self.absolute_offset;
        let body_offset = self.absolute_offset + at1;
        let tail_offset = self.absolute_offset + at2;

        let s = &mut self.slice;
        let (head, tmp) = s.split_at_mut(at1);
        let (body, tail) = tmp.split_at_mut(at2 - at1);

        let head = BinarySliceMut::new(head, head_offset);
        let body = BinarySliceMut::new(body, body_offset);
        let tail = BinarySliceMut::new(tail, tail_offset);

        return Ok((head, body, tail));
    }

    pub fn interpret_next_mut<T>(&'a mut self) -> InterpretResultMut<'a, T>
    where
        T: VOWrapU8ArrayMut,
    {
        let ret = self.interpret_relat_mut(0);
        if let InterpretResultMut::Valid(ref head, _, _) = ret {
            debug_assert_eq!(head.len(), 0);
        } else if let InterpretResultMut::Invalid(ref head, _, _) = ret {
            debug_assert_eq!(head.len(), 0);
        }

        return ret;
    }

    /// Interpret as T at `at` where `at` is absolute offset.
    pub fn interpret_abs_mut<T>(&'a mut self, at: usize) -> InterpretResultMut<'a, T>
    where
        T: VOWrapU8ArrayMut,
    {
        if at < self.absolute_offset {
            return InterpretResultMut::Error(InterpretErr::InvalidRequiredRangeBound);
        }

        let relative_offset = at - self.absolute_offset;
        self.interpret_relat_mut(relative_offset)
    }

    /// Interpret as T at `at` where `at` is relative offset.
    pub fn interpret_relat_mut<T>(&'a mut self, at: usize) -> InterpretResultMut<'a, T>
    where
        T: VOWrapU8ArrayMut,
    {
        let (head, body, tail) = match self.split_at_2_mut(at, at + T::size()) {
            Err(e) => {
                return InterpretResultMut::Error(e);
            }
            Ok(x) => x,
        };

        let to_interpret_mut = |inner: &'a mut T| InterpretMut {
            offset: body.absolute_offset,
            inner,
        };

        match T::constitude_mut(body.slice) {
            VOConstitudeResultMut::Valid(body, remain) => {
                debug_assert_eq!(remain.len(), 0);
                InterpretResultMut::Valid(head, to_interpret_mut(body), tail)
            }
            VOConstitudeResultMut::Invalid(body, remain) => {
                debug_assert_eq!(remain.len(), 0);
                InterpretResultMut::Invalid(head, to_interpret_mut(body), tail)
            }
            VOConstitudeResultMut::Error(_slice) => {
                debug_assert!(false);
                InterpretResultMut::Error(InterpretErr::NotEnoughLength)
            }
        }
    }
}

/*
    pub fn interpret_at<'a, T>(&'a mut self, at: usize) -> InterpretRef<'a, T>
    where
        T: VOWrapU8ArrayRef,
    {
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
*/

#[cfg(test)]
mod tests {
    use super::*;
    use crate::define_enchanted_type;
    use crate::types::repr_u8::Cu16;

    define_enchanted_type!(
        MockType,
        Cu16,
        Cu16::from_inner,
        [MOCK_VAL1, 1],
        [MOCK_VAL2, 256],
    );

    #[test]
    fn mut_split_at_mut() {
        let mut bw = BinaryWrapper::from_vec(vec![1, 2, 3]);
        let mut slice = bw.to_mut_slice();

        let (left, right) = slice.split_at_mut(1).unwrap();
        assert_eq!(left, BinarySliceMut::new(&mut [1], 0));
        assert_eq!(right, BinarySliceMut::new(&mut [2, 3], 1,));
    }

    #[test]
    fn mut_instatiate_mut() {
        let mut bw = BinaryWrapper::from_vec(vec![0, 1, 2, 3]);
        let mut slice = bw.to_mut_slice();

        let (head, mt, remain) = slice
            .interpret_next_mut::<MockType>()
            .ignore_validity()
            .unwrap();
        assert_eq!(head.slice, &mut []);
        assert_eq!(mt.inner, &MockType::MOCK_VAL2);
        assert_eq!(mt.offset, 0);
        assert_eq!(remain, BinarySliceMut::new(&mut [2, 3], 2));

        *mt.inner = MockType::MOCK_VAL1;

        assert_eq!(bw, BinaryWrapper::from_vec(vec![1, 0, 2, 3]));
    }

    #[test]
    fn mut_abs_access() {
        let mut bw = BinaryWrapper::from_vec(vec![0xff, 0xff, 0xfe, 0x01, 0x00, 0xff, 0xff, 0xff]);
        let mut slice = bw.to_mut_slice();

        let (mut head, body, tail) = slice.interpret_abs_mut::<MockType>(3).unwrap_valid();
        assert_eq!(tail.slice, &[0xff; 3]);

        assert_eq!(body.inner, &MockType::MOCK_VAL1);
        let (head, body, tail) = head.interpret_next_mut().ignore_validity().unwrap();
        assert_eq!(head.len(), 0);
        *body.inner = MockType::MOCK_VAL2;
        assert_eq!(tail.slice, &mut [0xfe]);
        tail.slice[0] = 0x12;

        assert_eq!(
            bw,
            BinaryWrapper::from_vec(vec![0x00, 0x01, 0x12, 0x01, 0x00, 0xff, 0xff, 0xff])
        );
    }
}
