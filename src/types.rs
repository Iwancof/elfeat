pub mod model;
pub mod primitive;

/// Error type of represent from u8.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FromU8Error<T> {
    /// The slice is too short to represent.
    NotEnoughSlice,

    /// The slice is invalid to represent.
    /// If you want to return a value regardless of success, return Some,
    InvalidValue(Option<T>),
}

impl<T> FromU8Error<T> {
    pub fn into<U>(self) -> FromU8Error<U>
    where
        T: Into<U>,
    {
        match self {
            FromU8Error::NotEnoughSlice => FromU8Error::NotEnoughSlice,
            FromU8Error::InvalidValue(x) => FromU8Error::InvalidValue(match x {
                Some(x) => Some(x.into()),
                None => None,
            }),
        }
    }
}

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
    fn from_slice(slice: &[u8]) -> Result<(usize, Self), FromU8Error<Self>>;
    fn to_slice(&self) -> Box<[u8]>;
}
