#[macro_export]
macro_rules! define_prim_wrap {
    ($name: ident, $inner_type: ty, $([$cons: ident, $val: expr],)*) => {
        #[repr(C)]
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

    };
}
