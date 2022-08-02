#[macro_export]
macro_rules! define_enchanted_type {
    ($name: ident, $inner_type: ty, $([$cons: ident, $val: expr],)*) => {
        #[repr(transparent)]
        #[derive(Copy, Clone, PartialEq, Eq)]
        struct $name {
            inner: $inner_type,
        }

        $(impl $name {
            const $cons: Self = Self { inner: $val };
        })*

        impl core::fmt::Display for $name {
            fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                let s = match self {
                    $(&Self::$cons => stringify!($cons),)*
                    _ => "Unknown",
                };

                write!(fmt, "{}", s)
            }
        }

        impl core::fmt::Debug for $name {
            fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(fmt, "{}", self.inner)
            }
        }

        impl $name {
            fn convert(val: &$inner_type) -> Result<&Self, &Self> {
                // Safety: ... ???
                let s = unsafe { &*(val as *const $inner_type as *const Self) };
                $(
                if s == &Self::$cons {
                    return Ok(s);
                };
                )*

                return Err(s);
            }
            fn constitude(slice: &[u8]) -> Result<(Result<&Self, &Self>, &[u8]), &[u8]> {
                let (read, next): (&$inner_type, &[u8]) = <$inner_type as crate::types::repr_u8::RepresentU8Array>::constitude(slice)?;

                Ok((Self::convert(read), next))
            }
        }
    };
}
