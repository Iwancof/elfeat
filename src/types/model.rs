use super::{Array, FromU8Array};

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
}

/// Define modeled types.
/// This take one inner type and may constant values.
/// contant values format are
/// ```rust
/// struct_vis struct StructName(InnerType),
/// [
///     vis name: val
/// ]
/// ```
/// or, C header style
///
/// ```rust
/// struct_vis struct StructName(InnerType),
/// member_vis
/// [
///     #define name val
/// ]
/// ```
/// In any cases, it can take attributes(struct and member).
/// If you provide "display_implementation = true" at last, this macro implements Display
/// trait(pretty print)
#[macro_export]
macro_rules! define_model_type {
    (
        $(#[$struct_meta: meta])*
        $vis: vis struct $struct_name: ident (
            $(#[$member_meta: meta])*
            $inner_type: ty
        ),
        [
            $(
                $mvis: vis ($name: ident: $val: expr),
            )*
        ]
    ) => {
        $(#[$struct_meta])*
        $vis struct $struct_name(
            $(#[$member_meta])*
            $inner_type
        );

        impl $struct_name {
            $(
                $mvis const $name: Self = Self($val);
            )*
        }

        impl crate::types::model::ModelFromU8Array for $struct_name {
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

        impl core::fmt::Debug for $struct_name {
            fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                $(
                    if self == &Self::$name {
                        return write!(fmt, "{} {{ {}({:?}) }}", core::any::type_name::<$struct_name>(), stringify!($name), self.0);
                    }
                )*

                return write!(fmt, "{} {{ {}({:?}) }}", core::any::type_name::<$struct_name>(), "Unknown", self.0);
            }
        }
    };
    (
        $(#[$struct_meta: meta])*
        $vis: vis struct $struct_name: ident (
            $(#[$member_meta: meta])*
            $inner_type: ty
        ),
        [
            $(
                $mvis: vis ($name: ident: $val: expr),
            )*
        ]
        display_implementation = true
    ) => {
        define_model_type!(
            $(#[$struct_meta])*
            $vis struct $struct_name (
                $(#[$member_meta])*
                $inner_type
            ),
            [
                $(
                    $mvis ($name: $val),
                )*
            ]
        );
        impl core::fmt::Display for $struct_name {
            fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                let _width = fmt.width().unwrap_or_else(|| 0);

                $(
                    if self == &Self::$name {
                        return write!(fmt, "{}({:?})", stringify!($name), self.0);
                    }
                )*

                return write!(fmt, "{}({:?})", "Unknown", self.0);
            }
        }
    };
    (
        $(#[$struct_meta: meta])*
        $vis: vis struct $struct_name: ident (
            $(#[$member_meta: meta])*
            $inner_type: ty
        ),
        $mvis: vis
        [
            $(
                #define $name: ident $val: expr
            )*
        ]
        $($extra_data: tt)*
    ) => {
        define_model_type!(
            $(#[$struct_meta])*
            $vis struct $struct_name(
                $(#[$member_meta])*
                $inner_type
            ),
            [
                $(
                    $mvis ($name: $val),
                )*
            ]
            $($extra_data)*
        );
    };
    (
        $(#[$struct_meta: meta])*
        $vis: vis struct $struct_name: ident (
            $(#[$member_meta: meta])*
            $inner_type: ty
        ),
        $mvis: vis
        [
            $(
                ($name: ident: $val: expr),
            )*
        ]
        $($extra_data: tt)*
    ) => {
        define_model_type!(
            $(#[$struct_meta])*
            $vis struct $struct_name(
                $(#[$member_meta])*
                $inner_type
            ),
            [
                $(
                    $mvis ($name: $val),
                )*
            ]
            $($extra_data)*
        );
    };
}

/// Define composed types.
/// This take modeled types as inner types.
/// Each members are Option<$mtype>.
/// ```rust
/// define_composed_type!(
///     pub struct  StructName {
///         member1: Option<Type1>, // Type1 is implements ModelFromU8Array.
///         member2: Option<Type2>, // Same.
///     },
/// );
///
/// If you provide "display_implementation = true" at last, this macro implements Display trait.
#[macro_export]
macro_rules! define_composed_type {
    (
        $(#[$struct_meta: meta])*
        $vis: vis struct $struct_name: ident {
            $(
                $(#[$member_meta: meta])*
                $mvis: vis $member: ident: Option<$mtype: ty>,
            )*
        },
        display_implementation = true
    ) => {
        define_composed_type!(
            $(#[$struct_meta])*
            $vis struct $struct_name {
                $(
                    $(#[$member_meta])*
                    $mvis $member: Option<$mtype>,
                )*
            }
        );
        impl core::fmt::Display for $struct_name {
            fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                let width = fmt.width().unwrap_or_else(|| 0);
                let next_width = width + crate::types::model::NEST_DEPTH;

                writeln!(fmt, "{}:", core::any::type_name::<$struct_name>())?;

                $(
                    write!(fmt, "{}{} = ", " ".repeat(next_width), stringify!($member))?;
                    paste::paste! {
                        self.[<write_ $member _to_fmt>](fmt)?;
                    }
                    writeln!(fmt, ",")?;
                )*

                Ok(())
            }
        }
    };
    (
        $(#[$struct_meta: meta])*
        $vis: vis struct $struct_name: ident {
            $(
                $(#[$member_meta: meta])*
                $mvis: vis $member: ident: Option<$mtype: ty>,
            )*
        }
    ) => {
        $(#[$struct_meta])*
        $vis struct $struct_name {
            $(
                $(#[$member_meta])*
                $mvis $member: Option<$mtype>,
            )*
        }
        impl $struct_name {
            $(
                paste::paste! {
                    // private
                    #[allow(unused)]
                    fn [<write_ $member _to_fmt>](&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        match &self.$member {
                            Some(s) => write!(fmt, "{}", s),
                            None => write!(fmt, "{}", "None")
                        }
                    }
                    #[allow(unused)]
                    fn [<write_ $member _to_fmt_debug>](&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        match &self.$member {
                            Some(s) => write!(fmt, "{:?}", s),
                            None => write!(fmt, "{}", "None")
                        }
                    }
                    /// If $member is None, this method will panic.
                    /// Otherwise, this returns $member's reference.
                    #[allow(unused)]
                    pub fn [<get_ $member _unwrap>](&self) -> &$mtype {
                        self.$member.as_ref().unwrap()
                    }
                    /// If $member is None, this method will panic.
                    /// Otherwise, this returns $member's `mut` reference.
                    #[allow(unused)]
                    pub fn [<get_ $member _unwrap_mut>](&mut self) -> &mut $mtype {
                        self.$member.as_mut().unwrap()
                    }
                    /// If $member is Some, returns true.
                    #[allow(unused)]
                    pub fn [<is_some_ $member>](&self) -> bool {
                        self.$member.is_some()
                    }
                    /// If $member is sanity, reutrns true.
                    /// If $member is None, returns false.
                    #[allow(unused)]
                    pub fn [<is_sanity_ $member>](&self) -> bool {
                        use crate::types::model::ModelFromU8Array;
                        match &self.$member {
                            Some(x) => x.is_sanity(),
                            None => false,
                        }
                    }
                }
            )*

            /// Return self instance that members are None.
            pub fn get_none() -> Self {
                Self {
                    $(
                        $member: None,
                    )*
                }
            }
        }


        impl crate::types::FromU8Array for $struct_name {
            #[allow(unused_assignments)] // for last slice assignment
            fn from_slice(mut slice: &[u8]) -> Result<(usize, Self), crate::types::FromU8Error<Self>> {
                use crate::types::FromU8Error;
                let mut ret = Self::get_none();
                let mut is_valid = true;

                paste::paste! {
                    let mut total = 0;
                    $(
                        let result = <$mtype>::from_slice(slice);
                        let (read, val): (usize, Option<$mtype>) = match result {
                            Ok(x) => {
                                (x.0, Some(x.1))
                            }
                            Err(e) => match e {
                                FromU8Error::NotEnoughSlice(_) => {
                                    return Err(FromU8Error::NotEnoughSlice(Some(ret)));
                                },
                                FromU8Error::InvalidValue((read, val)) => {
                                    is_valid = false;
                                    (read, val)
                                }
                            }
                        };
                        total += read;
                        slice = &slice[read..];

                        ret.$member = val;
                    )*

                    if is_valid {
                        Ok((total, ret))
                    } else {
                        Err(
                            crate::types::FromU8Error::InvalidValue((total, Some(ret)))
                        )
                    }
                }
            }

            fn to_slice(&self) -> Box<[u8]> {
                unimplemented!();
            }
        }

        impl crate::types::model::ModelFromU8Array for $struct_name {
            fn is_sanity(&self) -> bool {
                $(
                    if !match &self.$member {
                        Some(x) => x.is_sanity(),
                        None => false
                    } {
                        return false;
                    }
                )*

                return true;
            }
        }

        impl core::fmt::Debug for $struct_name {
            fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(fmt, "{} {{", core::any::type_name::<$struct_name>())?;
                $(
                    write!(fmt, "{}: ", stringify!($member))?;
                    paste::paste! {
                        self.[<write_ $member _to_fmt_debug>](fmt)?;
                    }
                    write!(fmt, ", ")?;
                )*

                return write!(fmt, "}}");
            }
        }
    };
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
        ]
        display_implementation = true
    );
    define_model_type!(
        #[derive(Copy, Clone, PartialEq, Eq)]
        struct MT2(u128),
        pub
        [
            (VAL: 0x88),
        ]
        display_implementation = true
    );

    define_composed_type!(
        struct MockType {
            a: Option<Array<MT1, 3>>,
            v: Option<MT2>,
        },
        display_implementation = true
    );

    #[test]
    fn test_mock_type() {
        let mut data: Vec<u8> = vec![1, 2, 3, 4, 5, 6];
        data.extend_from_slice(&[1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8]);
        data.extend_from_slice(&[1, 2, 3, 4, 4, 3, 2, 1]);

        let slice = data.as_ref();

        let (size, mt) = MockType::from_slice(slice).unwrap();

        assert_eq!(size, (16 / 8) * 3 + (128 / 8));
        assert_eq!(
            mt.a,
            Some([0x0201.into(), 0x0403.into(), 0x0605.into()].into())
        );
        assert_eq!(mt.v, Some(0x08070605040302010807060504030201.into()));
        assert_eq!(mt.is_sanity(), false);
    }

    #[test]
    fn sanity_check() {
        let mut d = MockType {
            a: Some([MT1::VAL, MT1::VAL, MT1::VAL].into()),
            v: Some(MT2::VAL),
        };

        assert_eq!(d.is_sanity(), true);

        d.get_a_unwrap_mut()[0] = 10.into();

        assert_eq!(d.is_sanity_a(), false);
        assert_eq!(d.is_sanity_v(), true);
        assert_eq!(d.is_sanity(), false);
    }
}
