use crate::types::{model::ModelFromU8Array, FromU8Array, FromU8Error};

/// Sequential binary wrapper. for instance, binary file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sequential {
    vector: Vec<u8>,
}

impl Sequential {
    /// Create instance from Vec<u8>.
    pub fn from_vec(vector: Vec<u8>) -> Self {
        Self { vector }
    }

    /// Create Seekable instance at `pos`
    pub fn to_seeakble_at(&self, pos: usize) -> Seekable<'_> {
        Seekable { inner: self, pos }
    }

    /// Create Seekable instance at 0
    pub fn to_seeakble(&self) -> Seekable<'_> {
        self.to_seeakble_at(0)
    }
}

/// Seekable binary wrapper.
/// This provide binary interpret methods.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Seekable<'a> {
    inner: &'a Sequential,
    pos: usize,
}

/// Raw InterpretResult. this contains original position and interpret result.
/// The type of result has size and object or error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InterpretResult<T> {
    pos: usize,
    val: Result<(usize, T), FromU8Error<T>>,
}

impl<T> InterpretResult<T> {
    /// Transform into tuple.
    pub fn to_tuple(self) -> (usize, Result<(usize, T), FromU8Error<T>>) {
        (self.pos, self.val)
    }

    /// From position and result.
    pub fn new(pos: usize, val: Result<(usize, T), FromU8Error<T>>) -> Self {
        Self { pos, val }
    }

    /// Unwrap self.val and forget object size.
    /// TODO: forgeting object size is good for us?
    pub fn to_tuple_unwrap(self) -> (usize, T)
    where
        T: core::fmt::Debug,
    {
        (self.pos, self.val.unwrap().1)
    }

    /// Unwrap self.val and forget object size.
    /// TODO: Same
    pub fn to_obj(self) -> InterpretObject<T>
    where
        T: core::fmt::Debug,
    {
        InterpretObject {
            pos: self.pos,
            val: self.val.unwrap().1,
        }
    }
}

/// InterpretObject is unwraped interpret result.
/// this contains original position and interpreted object.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InterpretObject<T> {
    pos: usize,
    val: T,
}

impl<T> InterpretObject<T> {}

impl Seekable<'_> {
    /// Interpret object at absolute offset at `apos`.
    pub fn interpret_abs_pos<InterpretType>(&self, apos: usize) -> InterpretResult<InterpretType>
    where
        InterpretType: FromU8Array,
    {
        InterpretResult::new(apos, InterpretType::from_slice(&self.inner.vector[apos..]))
    }

    /// Interpret object at self.pos. and proceed self.pos by interpreted object size.
    pub fn interpret_next<InterpretType>(&mut self) -> InterpretResult<InterpretType>
    where
        InterpretType: FromU8Array,
    {
        let (pos, result) = self.interpret_abs_pos::<InterpretType>(self.pos).to_tuple();

        if let Ok((read, _)) = result {
            self.pos += read;
        }
        if let Err(FromU8Error::InvalidValue((read, _))) = result {
            self.pos += read;
        }

        return InterpretResult::new(pos, result);
    }

    /// Seek to required offset.
    pub fn seek(&mut self, at: usize) -> Self {
        self.pos = at;
        return *self;
    }
}
