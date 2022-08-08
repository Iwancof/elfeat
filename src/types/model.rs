use super::primitive::{FromU8Array, FromU8Error};

pub trait ModelFromU8Array: FromU8Array {
    fn is_sanity(&self) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct MockType {
        a: [i16; 3],
        v: u128,
    }

    impl FromU8Array for MockType {
        fn from_slice(mut slice: &[u8]) -> Result<(usize, Self), FromU8Error<Self>> {
            let mut total = 0;
            let (read, tmp_a) = <[i16; 3]>::from_slice(slice).map_err(|e| match e {
                FromU8Error::NotEnoughSlice => FromU8Error::NotEnoughSlice,
                FromU8Error::InvalidValue(_) => FromU8Error::InvalidValue(None),
            })?;
            total += read;
            slice = &slice[read..];
            let (read, tmp_v) = u128::from_slice(slice).map_err(|e| match e {
                FromU8Error::NotEnoughSlice => FromU8Error::NotEnoughSlice,
                FromU8Error::InvalidValue(_) => FromU8Error::InvalidValue(None),
            })?;
            total += read;
            slice = &slice[read..];

            Ok((total, Self { a: tmp_a, v: tmp_v }))
        }

        fn to_slice(&self) -> Box<[u8]> {
            unimplemented!();
        }
    }

    impl ModelFromU8Array for MockType {
        fn is_sanity(&self) -> bool {
            true
        }
    }

    #[test]
    fn test_mock_type() {
        let mut data: Vec<u8> = vec![1, 2, 3, 4, 5, 6];
        data.extend_from_slice(&[1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8]);
        data.extend_from_slice(&[1, 2, 3, 4, 4, 3, 2, 1]);

        let slice = data.as_ref();

        let (size, mt) = MockType::from_slice(slice).unwrap();

        assert_eq!(size, (16 / 8) * 3 + (128 / 8));
        assert_eq!(mt.a, [0x0201, 0x0403, 0x0605]);
        assert_eq!(mt.v, 0x08070605040302010807060504030201);
    }
}
