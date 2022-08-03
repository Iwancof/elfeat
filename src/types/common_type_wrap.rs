pub enum ConstitudeResult<'a, T> {
    Valid((&'a T, &'a [u8])),
    Invalid((&'a T, &'a [u8])),
    Error(&'a [u8]),
}
pub enum ConstitudeResultMut<'a, T> {
    Valid((&'a mut T, &'a mut [u8])),
    Invalid((&'a mut T, &'a mut [u8])),
    Error(&'a mut [u8]),
}

#[macro_export]
macro_rules! define_enchanted_type {
    // TODO: support C comment out
    ($name: ident, $inner_type: ty, $conv: path, $(#define $cons: ident $val: expr)*) => {
        define_enchanted_type!($name, $inner_type, $([$cons, $conv($val)],)*);
    };
    ($name: ident, $inner_type: ty, $(#define $cons: ident $val: expr)+) => {
        define_enchanted_type!($name, $inner_type, $([$cons, $val],)*);
    };
    ($name: ident, $inner_type: ty, $conv:path, $([$cons: ident, $val: expr],)*) => {
        define_enchanted_type!($name, $inner_type, $([$cons, $conv($val)],)*);
    };
    ($name: ident, $inner_type: ty, $([$cons: ident, $val: expr],)*) => {
        #[repr(transparent)]
        #[derive(Copy, Clone, PartialEq, Eq, ref_cast::RefCast)]
        pub struct $name {
            pub inner: $inner_type,
        }

        $(impl $name {
            #[allow(unused)]
            pub const $cons: Self = Self { inner: $val };
        })*

        impl core::fmt::Display for $name {
            fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                let s = self.try_str();
                write!(fmt, "{}", s)
            }
        }

        impl core::fmt::Debug for $name {
            fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                let s = self.try_str();
                write!(fmt, "{} {{ {:?}: {} }}", stringify!($name), self.inner, s)
            }
        }

        impl $name {
            #[allow(unused)]
            fn try_str(&self) -> &'static str {
                match self {
                    $(&Self::$cons => stringify!($cons),)*
                    _ => "Unknown",
                }
            }
            #[allow(unused)]
            pub fn wrap_ref(val: &$inner_type) -> &Self {
                use ref_cast::RefCast;
                Self::ref_cast(val)
            }
            #[allow(unused)]
            pub fn wrap_ref_mut(val: &mut $inner_type) -> &mut Self {
                use ref_cast::RefCast;
                Self::ref_cast_mut(val)
            }
        }

        impl crate::types::repr_u8::VOWrapU8Array for $name {
            fn is_sanity(&self) -> bool {
                $(
                    if self == &Self::$cons {
                        return true;
                    };
                )*

                false
            }
        }
        impl crate::types::repr_u8::VOWrapU8ArrayRef for $name {
            fn constitude<'a>(slice: &'a [u8]) -> crate::types::repr_u8::VOConstitudeResult<'a, Self> {
                use crate::types::repr_u8::RepresentU8Array;
                use crate::types::repr_u8::VOWrapU8Array;
                let result = <$inner_type>::raw_constitude(slice);

                let (val, next) = match result {
                    Err(e) => return crate::types::repr_u8::VOConstitudeResult::Error(e),
                    Ok(v) => v,
                };

                let val = Self::wrap_ref(val);

                if Self::is_sanity(val) {
                    return crate::types::repr_u8::VOConstitudeResult::Valid(val, next);
                } else {
                    return crate::types::repr_u8::VOConstitudeResult::Invalid(val, next);
                }
            }
        }
        impl crate::types::repr_u8::VOWrapU8ArrayMut for $name {
            fn constitude_mut<'a>(slice: &'a mut [u8]) -> crate::types::repr_u8::VOConstitudeResultMut<'a, Self> {
                use crate::types::repr_u8::RepresentU8Array;
                use crate::types::repr_u8::VOWrapU8Array;
                let result = <$inner_type>::raw_constitude_mut(slice);

                let (val, next) = match result {
                    Err(e) => return crate::types::repr_u8::VOConstitudeResultMut::Error(e),
                    Ok(v) => v,
                };
                let val = Self::wrap_ref_mut(val);

                if Self::is_sanity(val) {
                    return crate::types::repr_u8::VOConstitudeResultMut::Valid(val, next);
                } else {
                    return crate::types::repr_u8::VOConstitudeResultMut::Invalid(val, next);
                }
            }
        }
    };
}
