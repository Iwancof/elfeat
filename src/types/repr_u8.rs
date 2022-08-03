// Primitive value

use crate::libc;
use core::mem::size_of;

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

    fn raw_constitude<'a>(slice: &'a [u8]) -> Result<(&'a Self, &'a [u8]), &'a [u8]> {
        let (left, right) = Self::split_at_self_size(slice)?;
        let s = Self::from_slice(left);

        Ok((s, right))
    }
    fn raw_constitude_mut<'a>(
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

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ReprCArray<T, const N: usize> {
    inner: [T; N],
}

unsafe impl<T, const N: usize> RepresentU8Array for ReprCArray<T, N> where T: RepresentU8Array {}
impl<T, const N: usize> ReprCArray<T, N> {
    pub const fn new(inner: [T; N]) -> Self {
        Self { inner }
    }
}

impl<T, const N: usize> AsMut<[T; N]> for ReprCArray<T, N> {
    fn as_mut(&mut self) -> &mut [T; N] {
        &mut self.inner
    }
}
impl<T, const N: usize> AsRef<[T; N]> for ReprCArray<T, N> {
    fn as_ref(&self) -> &[T; N] {
        &self.inner
    }
}

pub enum VOConstitudeResult<'a, T> {
    Valid(&'a T, &'a [u8]),
    Invalid(&'a T, &'a [u8]),
    Error(&'a [u8]),
}
pub enum VOConstitudeResultMut<'a, T> {
    Valid(&'a mut T, &'a mut [u8]),
    Invalid(&'a mut T, &'a mut [u8]),
    Error(&'a mut [u8]),
}

// TODO: Custom derive
pub trait VOWrapU8Array
where
    Self: Sized,
{
    fn size() -> usize {
        core::mem::size_of::<Self>()
    }
    fn is_sanity(&self) -> bool;
}

pub trait VOWrapU8ArrayRef
where
    Self: Sized + VOWrapU8Array,
{
    fn constitude<'a>(slice: &'a [u8]) -> VOConstitudeResult<'a, Self>;
}

pub trait VOWrapU8ArrayMut
where
    Self: Sized + VOWrapU8Array,
{
    fn constitude_mut<'a>(slice: &'a mut [u8]) -> VOConstitudeResultMut<'a, Self>;
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
    };
}

define_prim_type_mem_repr!(Cu16, u16);

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn cu16_from_slice_ok() {
        let slice: &[u8] = &[1, 0, 1, 2];
        let (val, tail) = Cu16::raw_constitude(slice).expect("convertion error");

        assert_eq!(tail, &[1, 2]);
        assert_eq!(val.inner, 1);
    }

    #[test]
    fn cu16_from_slice_length_err() {
        let slice: &[u8] = &[1];
        assert!(Cu16::raw_constitude(slice).is_err())
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

define_prim_type_mem_repr!(Cu8, u8);
define_prim_type_mem_repr!(Cu32, u32);
define_prim_type_mem_repr!(Cu64, u64);
define_prim_type_mem_repr!(Ci8, i8);
define_prim_type_mem_repr!(Ci16, i16);
define_prim_type_mem_repr!(Ci32, i32);
define_prim_type_mem_repr!(Ci64, i64);
define_prim_type_mem_repr!(Cchar, libc::c_char);

define_prim_type_mem_repr!(Csize, usize);

// TODO: exmaple implementation for uleb128
