use crate::types::{model::ModelFromU8Array, FromU8Error};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sequential {
    vector: Vec<u8>,
}

impl Sequential {
    pub fn from_vec(vector: Vec<u8>) -> Self {
        Self { vector }
    }
    pub fn to_seeakble_at(&self, pos: usize) -> Seekable<'_> {
        Seekable { inner: self, pos }
    }
    pub fn to_seeakble(&self) -> Seekable<'_> {
        self.to_seeakble_at(0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Seekable<'a> {
    inner: &'a Sequential,
    pos: usize,
}

impl Seekable<'_> {
    pub fn interpret_abs_pos<InterpretType>(
        &self,
        apos: usize,
    ) -> Result<(usize, InterpretType), FromU8Error<InterpretType>>
    where
        InterpretType: ModelFromU8Array,
    {
        InterpretType::from_slice(&self.inner.vector[apos..])
    }
}
