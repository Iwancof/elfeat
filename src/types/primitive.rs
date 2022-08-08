use super::{FromU8Array, FromU8Error};

impl<T, const N: usize> FromU8Array for [T; N]
where
    T: FromU8Array,
{
    fn from_slice(mut slice: &[u8]) -> Result<(usize, Self), FromU8Error<Self>> {
        use array_macro::array;
        let mut total = 0;

        let s = array![
            _ => {
                let (read, t) = T::from_slice(slice).map_err(|e| {
                    match e {
                        FromU8Error::NotEnoughSlice => FromU8Error::NotEnoughSlice,
                        FromU8Error::InvalidValue(_) => FromU8Error::InvalidValue(None),
                        // FIXME return some value.
                    }
                })?;

                total += read;
                slice = &slice[read..];

                t
            } ; N
        ];

        Ok((total, s))
    }
    fn to_slice(&self) -> Box<[u8]> {
        let mut ret = Vec::new();

        for obj in self {
            ret.extend_from_slice(&obj.to_slice());
        }

        ret.into_boxed_slice()
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn i32_from_slice_ok() {
        let mut slice: &[u8] = &[1, 2, 3, 4, 1, 2];
        let val = i32::from_slice_consume(&mut slice).expect("convertion error");

        assert_eq!(slice, &[1, 2]);
        assert_eq!(val, 0x04030201);
    }

    #[test]
    fn i32_from_slice_length_err() {
        let slice: &[u8] = &[1];

        let e = i32::from_slice(slice).expect_err("Expected error");
        assert_eq!(e, FromU8Error::NotEnoughSlice);
    }

    #[test]
    fn i32_to_slice_represent() {
        let val = 0x12345678;
        assert_eq!(val.to_slice().as_ref(), &[0x78, 0x56, 0x34, 0x12]);
    }

    #[test]
    fn i32_to_slice_change() {
        let val = 0x12345678;
        let mut slice = val.to_slice();

        slice[0] = 0xff;

        assert_eq!(i32::from_slice(slice.as_ref()).unwrap(), (4, 0x123456ff));
    }

    #[test]
    fn array_i32_from_slice_ok() {
        let mut slice: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let val = <[i32; 2]>::from_slice_consume(&mut slice).expect("convertion error");

        assert_eq!(val, [0x04030201, 0x08070605]);
        assert_eq!(slice, &[9, 10]);
    }

    #[test]
    fn array_i32_from_slice_length_error() {
        let mut slice: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let err = <[i32; 3]>::from_slice_consume(&mut slice).expect_err("expected error");

        assert_eq!(err, FromU8Error::NotEnoughSlice);
    }

    #[test]
    fn array_u32_to_slice_change() {
        let mut slice: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mut val = <[u32; 2]>::from_slice_consume(&mut slice).expect("convertion error");

        val[0] = 0xf1f2f3f4;
        val[1] = 0xf5f6f7f8;

        let s = val.to_slice();

        assert_eq!(
            s.as_ref(),
            &[0xf4, 0xf3, 0xf2, 0xf1, 0xf8, 0xf7, 0xf6, 0xf5]
        );
    }
}

#[macro_export]
macro_rules! impl_f8a_le_bytes {
    ($t: ty) => {
        impl crate::types::FromU8Array for $t {
            fn from_slice(slice: &[u8]) -> Result<(usize, Self), crate::types::FromU8Error<Self>> {
                use crate::types::FromU8Error;

                if slice.len() < core::mem::size_of::<Self>() {
                    return Err(FromU8Error::NotEnoughSlice);
                }
                let (array, _remain) = slice.split_array_ref();

                Ok((core::mem::size_of::<Self>(), Self::from_le_bytes(*array)))
            }

            fn to_slice(&self) -> Box<[u8]> {
                Box::new(Self::to_le_bytes(*self))
            }
        }
    };
}

impl_f8a_le_bytes!(i8);
impl_f8a_le_bytes!(i16);
impl_f8a_le_bytes!(i32);
impl_f8a_le_bytes!(i64);
impl_f8a_le_bytes!(i128);

impl_f8a_le_bytes!(u8);
impl_f8a_le_bytes!(u16);
impl_f8a_le_bytes!(u32);
impl_f8a_le_bytes!(u64);
impl_f8a_le_bytes!(u128);
