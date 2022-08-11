use super::{Array, FromU8Array};

#[macro_use]
pub mod macros;

/// Display nest width.
pub const NEST_DEPTH: usize = 4;

/// Modeled types.
/// This has sanityness e.g, file offset value is too big.
pub trait ModelFromU8Array: FromU8Array {
    /// Return true if value is valid.
    fn is_sanity(&self) -> bool;
}

/// Composed type members are Option<$mtype>.
/// We must provide all of members has value as Some(_)
pub trait ComposedFromU8Array: ModelFromU8Array {
    /// Return true if all of values are Some(_)
    fn is_some(&self) -> bool;

    fn is_none(&self) -> bool {
        !self.is_some()
    }
}

impl<T, const N: usize> ModelFromU8Array for Array<T, N>
where
    T: ModelFromU8Array,
{
    fn is_sanity(&self) -> bool {
        self.0.iter().all(|x| x.is_sanity())
    }
}
impl<T, const N: usize> core::fmt::Display for Array<T, N>
where
    T: core::fmt::Display,
{
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let width = fmt.width().unwrap_or_else(|| 0);
        let next_width = width + NEST_DEPTH;

        writeln!(fmt, "[{}; {}] {{", core::any::type_name::<T>(), N)?;

        for e in &self.0 {
            writeln!(fmt, "{}{},", " ".repeat(next_width), e)?;
        }

        write!(fmt, "{}}}", " ".repeat(width))
    }
}
impl<T, const N: usize> core::fmt::LowerHex for Array<T, N>
where
    T: core::fmt::LowerHex,
{
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let width = fmt.width().unwrap_or_else(|| 0);
        let next_width = width + NEST_DEPTH;

        writeln!(fmt, "[{}; {:x}] {{", core::any::type_name::<T>(), N)?;

        for e in &self.0 {
            writeln!(fmt, "{}{:x},", " ".repeat(next_width), e)?;
        }

        write!(fmt, "{}}}", " ".repeat(width))
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;

    define_model_type!(
        #[derive(Copy, Clone, PartialEq, Eq)]
        struct MT1(i16),
        [
            pub (VAL: 0),
        ],
        display = true,
        bitflags = false,
    );
    define_model_type!(
        #[derive(Copy, Clone, PartialEq, Eq)]
        struct MT2(u128),
        pub
        [
            (VAL: 0x88),
        ],
        display = true,
        bitflags = false,
    );

    define_model_type!(
        #[derive(Copy, Clone, PartialEq, Eq)]
        struct BF(u16),
        pub
        [
            (F1: 0b1),
            (F2: 0b10),
        ],
        display = true,
        bitflags = true,
    );

    define_composed_type!(
        struct MockType {
            a: Option<Array<MT1, 3>>,
            v: Option<MT2>,
            bf: Option<BF>,
        },
        display = true,
    );

    #[test]
    fn test_mock_type() {
        let mut data: Vec<u8> = vec![1, 2, 3, 4, 5, 6];
        data.extend_from_slice(&[1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8]);
        data.extend_from_slice(&[1, 2, 3, 4, 4, 3, 2, 1]);

        let slice = data.as_ref();

        let (size, mt) = MockType::from_slice(slice).unwrap();

        assert_eq!(size, (16 / 8) * 3 + (128 / 8) + (16 / 8));
        assert_eq!(
            mt.a,
            Some([0x0201.into(), 0x0403.into(), 0x0605.into()].into())
        );
        assert_eq!(mt.v, Some(0x08070605040302010807060504030201.into()));
        assert_eq!(mt.bf, Some(0x0201.into()));
        assert_eq!(mt.is_sanity(), false);
    }

    #[test]
    fn composed_to_slice_test() {
        let d = MockType {
            a: Some([MT1::VAL, MT1::VAL, MT1::VAL].into()),
            v: Some(MT2::VAL),
            bf: Some(BF(0b11)),
        };

        let slice = d.to_slice();
        assert_eq!(
            &[
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x88, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0b11, 0x00
            ],
            slice.as_ref()
        );
    }

    #[test]
    fn composed_to_slice_none() {
        let d = MockType {
            a: Some([MT1::VAL, MT1::VAL, MT1::VAL].into()),
            v: None,
            bf: Some(BF(0b11)),
        };

        let slice = d.to_slice();
        assert_eq!(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00,], slice.as_ref());
    }

    #[test]
    fn sanity_check() {
        let mut d = MockType {
            a: Some([MT1::VAL, MT1::VAL, MT1::VAL].into()),
            v: Some(MT2::VAL),
            bf: Some(BF(0b11)),
        };

        assert_eq!(d.is_sanity(), true);

        d.get_a_unwrap_mut()[0] = 10.into();
        *d.get_bf_unwrap_mut().inner_mut() |= 0b100;

        assert_eq!(d.is_sanity_a(), false);
        assert_eq!(d.is_sanity_v(), true);
        assert_eq!(d.is_sanity_bf(), false);
        assert_eq!(d.is_sanity(), false);
    }

    #[test]
    fn bitflags_to_string() {
        define_model_type!(
            #[derive(PartialEq, Eq)]
            struct TmpBF(u16),
            pub
            [
                (B0: 1 << 0),
                (B1: 1 << 1),
                (B2: 1 << 2),
            ],
            display = true, bitflags = true,
        );

        assert_eq!(TmpBF::new(0b0111).to_string(), "(B0 | B1 | B2)".to_string());
        assert_eq!(TmpBF::new(0b0110).to_string(), "(B1 | B2)".to_string());
        assert_eq!(TmpBF::new(0b0000).to_string(), "()".to_string());
        assert_eq!(TmpBF::new(0b1000).to_string(), "(Unknown(8))".to_string());
        assert_eq!(
            TmpBF::new(0b1001).to_string(),
            "(B0 | Unknown(8))".to_string()
        );
    }

    #[test]
    fn bitflags_on_off_get() {
        define_model_type!(
            #[derive(PartialEq, Eq)]
            struct TmpBF(u16),
            pub
            [
                (B0: 1 << 0),
                (B1: 1 << 1),
                (B2: 1 << 2),
            ],
            display = true, bitflags = true,
        );

        let mut bf = TmpBF(0);
        assert_eq!(bf.is_empty(), true);
        assert_eq!(bf.is_zero(), true);

        assert_eq!(bf.on_B0(), false);
        assert_eq!(bf.get_B0(), true);

        assert_eq!(bf.is_empty(), false);
        assert_eq!(bf.is_zero(), false);

        assert_eq!(bf.off_B0(), true);
        assert_eq!(bf.get_B0(), false);

        assert_eq!(bf.on_at(3), false);
        assert_eq!(bf.get_at(3), true);

        assert_eq!(bf.is_empty(), true);
        assert_eq!(bf.is_zero(), false);

        assert_eq!(bf.is_sanity(), false);
    }
}
