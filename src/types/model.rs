use super::FromU8Array;

pub trait ModelFromU8Array: FromU8Array {
    fn is_sanity(&self) -> bool;
}

macro_rules! define_model_type {
    (
    $(#[$struct_meta:meta])*
    $vis: vis struct $struct_name: ident ( $inner_type: ty ) ; [$( $mvis: vis ($name: ident: $val: expr),)*]) => {
        #[repr(transparent)]
        $(#[$struct_meta])*
        $vis struct $struct_name($inner_type);

        impl $struct_name {
            $(
            $mvis const $name: Self = Self($val);
            )*
        }

        impl ModelFromU8Array for $struct_name {
            fn is_sanity(&self) -> bool {
                $(
                    if self == &Self::$name {
                        return true;
                    }
                )*
                return false;
            }
        }

        impl Into<$inner_type> for $struct_name {
            fn into(self) -> $inner_type {
                self.0
            }
        }
        impl Into<$struct_name> for $inner_type {
            fn into(self) -> $struct_name {
                $struct_name(self)
            }
        }

        impl crate::types::FromU8Array for $struct_name {
            fn from_slice(slice: &[u8]) -> Result<(usize, Self), crate::types::FromU8Error<Self>> {
                match <$inner_type>::from_slice(slice) {
                    Ok((read, x)) => Ok((read, Self(x))),
                    Err(e) => Err(e.into()),
                }
            }
            fn to_slice(&self) -> Box<[u8]> {
                self.0.to_slice()
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;

    define_model_type!(
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    struct MockModel(i32); [
        (VAL: 0),
    ]);

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
