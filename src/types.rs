pub mod elf;
pub mod primitive;

#[macro_use]
pub mod model;

/// Array struct that is able to intepreted to &[u8]
/// The reason why we don't use [T; N] as FromU8Array is, to implement Display trait by user.
#[derive(Debug, PartialEq, Eq)]
pub struct Array<T, const N: usize>([T; N]);

impl<T, const N: usize> From<Array<T, N>> for [T; N] {
    fn from(x: Array<T, N>) -> Self {
        x.0
    }
}
impl<T, const N: usize> From<[T; N]> for Array<T, N> {
    fn from(x: [T; N]) -> Self {
        Self(x)
    }
}
impl<T, const N: usize> core::ops::Index<usize> for Array<T, N> {
    type Output = T;

    fn index(&self, idx: usize) -> &T {
        &self.0[idx]
    }
}
impl<T, const N: usize> core::ops::IndexMut<usize> for Array<T, N> {
    fn index_mut(&mut self, idx: usize) -> &mut T {
        &mut self.0[idx]
    }
}

impl<T, const N: usize> Array<T, N> {
    pub fn inner(&self) -> &[T; N] {
        &self.0
    }
}

/// Error type of represent from u8.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FromU8Error<T> {
    /// The slice is too short to represent.
    NotEnoughSlice(Option<T>),

    /// The slice is invalid to represent.
    /// If you want to return a value regardless of success, return Some,
    /// Left is read size.
    InvalidValue((usize, Option<T>)),
}

impl<T> FromU8Error<T> {
    pub fn into<U>(self) -> FromU8Error<U>
    where
        T: Into<U>,
    {
        match self {
            FromU8Error::NotEnoughSlice(x) => FromU8Error::NotEnoughSlice(match x {
                Some(x) => Some(x.into()),
                None => None,
            }),
            FromU8Error::InvalidValue((read, x)) => FromU8Error::InvalidValue(match x {
                Some(x) => (read, Some(x.into())),
                None => (read, None),
            }),
        }
    }
}

/// The train of type that has represent as u8 array.
pub trait FromU8Array
where
    Self: Sized,
{
    /// Read from slice and slide pointer.
    /// If this returns err(_), slice will not be changed.
    fn from_slice_consume(slice: &mut &[u8]) -> Result<Self, FromU8Error<Self>> {
        let (read, obj) = Self::from_slice(slice)?;
        *slice = &slice[read..];
        Ok(obj)
    }

    /// Interpret from slice.
    fn from_slice(slice: &[u8]) -> Result<(usize, Self), FromU8Error<Self>>;

    /// Transform to slice on heap.
    fn to_slice(&self) -> Box<[u8]>;
}
