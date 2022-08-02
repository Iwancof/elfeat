// Primitive value

use crate::libc;
use core::mem::{forget, size_of, transmute};

/// Safety constrains: if representing as u8 is safe.
pub unsafe trait RepresentU8Array
where
    Self: Sized + Copy,
{
    fn size() -> usize {
        size_of::<Self>()
    }
    fn split_at_self_size(slice: &[u8]) -> Result<(&[u8], &[u8]), &[u8]> {
        if slice.len() < Self::size() {
            return Err(slice);
        }
        Ok(slice.split_at(Self::size()))
    }

    fn split_at_self_size_mut(slice: &mut [u8]) -> Result<(&mut [u8], &mut [u8]), &mut [u8]> {
        if slice.len() < Self::size() {
            return Err(slice);
        }
        Ok(slice.split_at_mut(Self::size()))
    }

    fn constitude<'a>(slice: &'a [u8]) -> Result<(&'a Self, &'a [u8]), &'a [u8]> {
        let (left, right) = Self::split_at_self_size(slice)?;
        let s = Self::from_slice(left);

        Ok((s, right))
    }
    fn constitude_mut<'a>(
        slice: &'a mut [u8],
    ) -> Result<(&'a mut Self, &'a mut [u8]), &'a mut [u8]> {
        let (left, right) = Self::split_at_self_size_mut(slice)?;
        let s = Self::from_slice_mut(left);

        Ok((s, right))
    }
    fn from_slice<'a>(slice: &'a [u8]) -> &'a Self {
        // TODO: const &'a [u8; Self::size()] -> &'a Self.
        assert_eq!(slice.len(), Self::size());
        // Safety: slice's length is sizeof($inner).
        unsafe { &*(slice.as_ptr() as *const u8 as *const Self) }
    }
    fn to_slice<'a>(&'a self) -> &'a [u8] {
        // Safety: self's length is Self::size()
        unsafe { core::slice::from_raw_parts(self as *const Self as *const u8, Self::size()) }
    }
    fn from_slice_mut<'a>(slice: &'a mut [u8]) -> &'a mut Self {
        assert_eq!(slice.len(), Self::size());
        // Safety: slice's length is sizeof($inner).
        unsafe { &mut *(slice.as_mut_ptr() as *mut u8 as *mut Self) }
    }
    fn to_slice_mut<'a>(&'a mut self) -> &'a mut [u8] {
        // Safety: self's length is Self::size()
        unsafe { core::slice::from_raw_parts_mut(self as *mut Self as *mut u8, Self::size()) }
    }
}

macro_rules! define_prim_type_mem_repr {
    ($typename: ident, $inner: ty) => {
        #[repr(transparent)]
        #[derive(Clone, Copy, PartialEq, Eq, Debug)]
        pub struct $typename {
            inner: $inner,
        }

        unsafe impl RepresentU8Array for $typename {}

        impl $typename {
            pub const fn from_inner(inner: $inner) -> Self {
                Self { inner }
            }
            pub const fn into_inner(self) -> $inner {
                self.inner
            }
        }

        impl core::fmt::Display for $typename {
            fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(fmt, "{}", self.inner)
            }
        }
    };
}

define_prim_type_mem_repr!(Cu16, u16);

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn cu16_from_slice_ok() {
        let slice: &[u8] = &[1, 0, 1, 2];
        let (val, tail) = Cu16::constitude(slice).expect("convertion error");

        assert_eq!(tail, &[1, 2]);
        assert_eq!(val.inner, 1);
    }

    #[test]
    fn cu16_from_slice_length_err() {
        let slice: &[u8] = &[1];
        assert!(Cu16::constitude(slice).is_err())
    }

    #[test]
    fn cu16_to_slice_represent() {
        let val = Cu16::from_inner(0x1234);
        assert_eq!(val.to_slice(), &[0x34, 0x12]);
    }

    #[test]
    fn cu16_to_slice_change() {
        let mut val = Cu16::from_inner(0x1234);
        let slice: &mut [u8] = val.to_slice_mut();

        slice[0] = 0xff;

        assert_eq!(val.into_inner(), 0x12ff);
    }
}

define_prim_type_mem_repr!(Cu32, u32);
define_prim_type_mem_repr!(Cu64, u64);
define_prim_type_mem_repr!(Ci16, i16);
define_prim_type_mem_repr!(Ci32, i32);
define_prim_type_mem_repr!(Ci64, i64);
define_prim_type_mem_repr!(Cchar, libc::c_char);
