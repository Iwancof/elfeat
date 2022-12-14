#[macro_export]
macro_rules! define_model_type_bitflags {
    ($($strmeta: meta)*, $str_vis: vis, $strname: ident, $($_memmeta: meta)*, $memtype: ty,
     [],
     display = false,
    ) => {
        impl core::fmt::Debug for $strname {
            fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                return write!(fmt, "{} {{ ({:?}) }}", core::any::type_name::<$strname>(), self.0);
            }
        }
    };
    ($($strmeta: meta)*, $str_vis: vis, $strname: ident, $($_memmeta: meta)*, $memtype: ty,
     [],
     display = true,
    ) => {
        crate::define_model_type_bitflags!($($strmeta)*, $str_vis, $strname, $($_memmeta)*, $memtype,
                                    [
                                    ],
                                    display = false,
                                    );
        impl core::fmt::Display for $strname {
            fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                return write!(fmt, "{:?}", self.0);
            }
        }
    };
    ($($strmeta: meta)*, $str_vis: vis, $strname: ident, $($_memmeta: meta)*, $memtype: ty,
     [
        $(
            $memvis: vis ($name: ident: $val: expr),
        )*
     ],
     display = false,
    ) => {
        impl core::fmt::Debug for $strname {
            fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                let mut is_wrote = false;
                let mut copyed = Self::new(*self.inner_ref());

                write!(fmt, "{} {{ {}(", core::any::type_name::<$strname>(), self.inner_ref())?;

                paste::paste! {
                    $(
                        if copyed.[<off_ $name>]() {
                            if is_wrote {
                                write!(fmt, " | {}({})", stringify!($name), $val)?;
                            } else {
                                write!(fmt, "{}({})", stringify!($name), $val)?;
                            }
                            is_wrote = true;
                        }
                    )*
                }

                if !copyed.is_zero() {
                    if is_wrote {
                        write!(fmt, " | Unknown({})", copyed.inner())?;
                    } else {
                        write!(fmt, "Unknown({})", copyed.inner())?;
                    }
                }
                write!(fmt, ") }}")?;

                Ok(())
            }
        }
    };
    ($($strmeta: meta)*, $str_vis: vis, $strname: ident, $($_memmeta: meta)*, $memtype: ty,
     [
        $(
            $_memvis: vis ($name: ident: $val: expr),
        )*
     ],
     display = true,
    ) => {
        crate::define_model_type_bitflags!($($strmeta)*, $str_vis, $strname, $($_memmeta)*, $memtype,
                                    [
                                        $(
                                            $_memvis ($name: $val),
                                        )*
                                    ],
                                    display = false,
                                    );

        impl core::fmt::Display for $strname {
            fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                let mut is_wrote = false;
                let mut copyed = Self::new(*self.inner_ref());

                write!(fmt, "(")?;

                paste::paste! {
                    $(
                        if copyed.[<off_ $name>]() {
                            if is_wrote {
                                write!(fmt, " | {}", stringify!($name))?;
                            } else {
                                write!(fmt, "{}", stringify!($name))?;
                            }
                            is_wrote = true;
                        }
                    )*
                }

                if !copyed.is_zero() {
                    if is_wrote {
                        write!(fmt, " | Unknown({})", copyed.inner())?;
                    } else {
                        write!(fmt, "Unknown({})", copyed.inner())?;
                    }
                }
                write!(fmt, ")")?;

                Ok(())
            }
        }
    };
}

#[macro_export]
macro_rules! define_model_type_normal {
    ($($strmeta: meta)*, $str_vis: vis, $strname: ident, $($memmeta: meta)*, $memtype: ty,
     [],
     display = false,
    ) => {
        impl core::fmt::Debug for $strname {
            fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                return write!(fmt, "{} {{ ({:?}) }}", core::any::type_name::<$strname>(), self.0);
            }
        }
    };
    ($($strmeta: meta)*, $str_vis: vis, $strname: ident, $($memmeta: meta)*, $memtype: ty,
     [],
     display = true,
    ) => {
        crate::define_model_type_normal!($($strmeta)*, $str_vis, $strname, $($memmeta)*, $memtype,
                                    [],
                                    display = false,
                                    );
        impl core::fmt::Display for $strname {
            fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                return write!(fmt, "{:?}", self.0);
            }
        }
    };
    ($($strmeta: meta)*, $str_vis: vis, $strname: ident, $($memmeta: meta)*, $memtype: ty,
     [
        $(
            $memvis: vis ($name: ident: $val: expr),
        )*
     ],
     display = false,
    ) => {
        impl core::fmt::Debug for $strname {
            fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                $(
                    if self == &Self::$name {
                        return write!(fmt, "{} {{ {}({:?}) }}", core::any::type_name::<$strname>(), stringify!($name), self.0);
                    }
                )*

                return write!(fmt, "{} {{ {}({:?}) }}", core::any::type_name::<$strname>(), "Unknown", self.0);
            }
        }

    };
    ($($strmeta: meta)*, $str_vis: vis, $strname: ident, $($memmeta: meta)*, $memtype: ty,
     [
        $(
            $memvis: vis ($name: ident: $val: expr),
        )*
     ],
     display = true,
    ) => {
        crate::define_model_type_normal!($($strmeta)*, $str_vis, $strname, $($memmeta)*, $memtype,
                                    [
                                        $(
                                            $memvis ($name: $val),
                                        )*
                                    ],
                                    display = false,
                                    );

        impl core::fmt::Display for $strname {
            fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                $(
                    if self == &Self::$name {
                        return write!(fmt, "{}({:?})", stringify!($name), self.0);
                    }
                )*

                return write!(fmt, "{}({:?})", "Unknown", self.0);
            }
        }
    };
}

#[macro_export]
macro_rules! define_constants {
    ($($strmeta: meta)*, $str_vis: vis, $strname: ident, $($memmeta: meta)*, $memtype: ty,
     [
        $(
            $vis: vis ($name: ident: $val: expr),
        )*
     ],
     display = $display: tt,
     bitflags = true,
    ) => {
        $(#[$strmeta])*
        $str_vis struct $strname(
            $(#[$memmeta])*
            $memtype
        );

        crate::define_model_type_bitflags!($($strmeta)*, $str_vis, $strname, $($memmeta)*, $memtype,
                                    [
                                        $(
                                            $vis ($name: $val),
                                        )*
                                    ],
                                    display = $display,
                                    );

        impl crate::types::model::ModelFromU8Array for $strname {
            #[allow(unused_mut)]
            fn is_sanity(&self) -> bool {
                let mut copyed = Self::new(*self.inner_ref());
                paste::paste! {
                    $(
                        copyed.[<off_ $name>]();
                    )*
                }

                copyed.is_zero()
            }
        }

        paste::paste! {
            $(
                impl $strname {
                    #[allow(non_upper_case_globals)]
                    $vis const $name: $strname = $strname($val);

                    #[allow(unused)]
                    #[allow(non_snake_case)]
                    pub fn [<get_ $name>](&self) -> bool {
                        self.get(Self::$name)
                    }
                    #[allow(unused)]
                    #[allow(non_snake_case)]
                    pub fn [<set_ $name>](&mut self, val: bool) -> bool {
                        self.set(Self::$name, val)
                    }
                    #[allow(unused)]
                    #[allow(non_snake_case)]
                    pub fn [<on_ $name>](&mut self) -> bool {
                        self.on(Self::$name)
                    }
                    #[allow(unused)]
                    #[allow(non_snake_case)]
                    pub fn [<off_ $name>](&mut self) -> bool {
                        self.off(Self::$name)
                    }
                    #[allow(unused)]
                    #[allow(non_snake_case)]
                    pub fn [<is_ $name>](&self) -> bool {
                        self == &Self::$name
                    }
                }
            )*
        }

        impl core::ops::Not for $strname {
            type Output = Self;

            fn not(self) -> Self::Output {
                Self::new(!self.inner())
            }
        }

        impl core::ops::BitOr for $strname {
            type Output = Self;

            fn bitor(self, rhs: Self) -> Self::Output {
                Self::new(
                    self.inner() | rhs.inner()
                )
            }
        }

        impl core::ops::BitAnd for $strname {
            type Output = Self;

            fn bitand(self, rhs: Self) -> Self::Output {
                Self::new(
                    self.inner() & rhs.inner()
                )
            }
        }

        impl $strname {
            /// Return true if inner value is zero.
            /// If self has an uncovered value, this returns false.
            #[allow(unused)]
            pub fn is_zero(&self) -> bool {
                self.inner_ref() == &0
            }

            /// Return true if self dosen't have any constants.
            /// If self has only an uncovered value, this returns true.
            #[allow(unused)]
            pub fn is_empty(&self) -> bool {
                paste::paste! {
                    $(
                        if self.[<get_ $name>]() {
                            return false;
                        };
                    )*
                }
                return true;
            }

            /// Set value(bool) at pos(Self)
            /// The return value is previous.
            #[allow(unused)]
            pub fn set(&mut self, pos: Self, val: bool) -> bool {
                let old = self.get(Self::new(*pos.inner_ref()));
                if val {
                    *self = Self::new(*self.inner_ref()) | pos;
                } else {
                    *self = Self::new(*self.inner_ref()) & (!pos);
                }
                old
            }

            /// Set value(bool) at pos(usize)
            /// The return value is previous.
            #[allow(unused)]
            pub fn set_at(&mut self, at: usize, val: bool) -> bool {
                let pos = Self::new(1 << at);
                self.set(pos, val)
            }

            /// Get value at pos(Self)
            #[allow(unused)]
            pub fn get(&self, pos: Self) -> bool {
                (Self::new(*self.inner_ref()) & pos).inner() != 0
            }

            /// Get value at pos(usize)
            #[allow(unused)]
            pub fn get_at(&self, at: usize) -> bool {
                let pos = Self::new(1 << at);
                self.get(pos)
            }

            /// Toggle value at pos(Self)
            #[allow(unused)]
            pub fn toggle(&mut self, pos: Self) -> bool {
                self.set(Self::new(*pos.inner_ref()), !self.get(pos))
            }

            /// Toggle value at pos(usize)
            #[allow(unused)]
            pub fn toggle_at(&mut self, at: usize) -> bool {
                let pos = Self::new(1 << at);
                self.toggle(pos)
            }

            /// Turn on at pos(Self)
            #[allow(unused)]
            pub fn on(&mut self, pos: Self) -> bool {
                self.set(pos, true)
            }

            /// Turn on at pos(usize)
            #[allow(unused)]
            pub fn on_at(&mut self, at: usize) -> bool {
                let pos = Self::new(1 << at);
                self.on(pos)
            }

            /// Turn off at pos(Self)
            #[allow(unused)]
            pub fn off(&mut self, pos: Self) -> bool {
                self.set(pos, false)
            }

            /// Turn off at pos(usize)
            #[allow(unused)]
            pub fn off_at(&mut self, at: usize) -> bool {
                let pos = Self::new(1 << at);
                self.on(pos)
            }
        }
    };
    ($($strmeta: meta)*, $str_vis: vis, $strname: ident, $($memmeta: meta)*, $memtype: ty,
     [
        $(
            $vis: vis ($name: ident: $val: expr),
        )*
     ],
     display = $display: tt,
     bitflags = false,
     ) => {
        $(#[$strmeta])*
        $str_vis struct $strname(
            $(#[$memmeta])*
            $memtype
        );
        crate::define_model_type_normal!($($strmeta)*, $str_vis, $strname, $($memmeta)*, $memtype,
                                  [
                                    $(
                                        $vis ($name: $val),
                                    )*
                                  ],
                                  display = $display,
                                  );

        impl crate::types::model::ModelFromU8Array for $strname {
            fn is_sanity(&self) -> bool {
                $(
                    if self == &Self::$name {
                        return true;
                    }
                )*
                return false;
            }
        }

        paste::paste! {
            $(
                impl $strname {
                    #[allow(non_upper_case_globals)]
                    $vis const $name: Self = Self($val);

                    #[allow(unused)]
                    #[allow(non_snake_case)]
                    pub fn [<is_ $name>](&self) -> bool {
                        self == &Self::$name
                    }
                }
            )*

            impl $strname {
                #[allow(unused)]
                pub fn is_constant(&self) -> bool {
                    $(
                        self.[<is_ $name>]() ||
                    )* false
                }
            }
        }
     };
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
        ],
        $($extra: tt)*
    ) => {
        crate::define_constants!(
            $($struct_meta)*, $vis, $struct_name, $($member_meta)*, $inner_type,
            [
                $(
                    $mvis ($name: $val),
                )*
            ],
            $($extra)*
        );

        impl crate::types::FromU8Array for $struct_name {
            fn from_slice(slice: &[u8]) -> Result<(usize, Self), crate::types::FromU8Error<Self>> {
                match <$inner_type>::from_slice(slice) {
                    Ok((read, x)) => Ok((read, Self::new(x))),
                    Err(e) => Err(e.into()),
                }
            }
            fn to_slice(&self) -> Box<[u8]> {
                self.inner_ref().to_slice()
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

        impl $struct_name {
            #[allow(unused)]
            pub fn inner(self) -> $inner_type {
                self.0
            }
            #[allow(unused)]
            pub fn inner_ref(&self) -> &$inner_type {
                &self.0
            }
            #[allow(unused)]
            pub fn inner_mut(&mut self) -> &mut $inner_type {
                &mut self.0
            }

            #[allow(unused)]
            pub fn new(val: $inner_type) -> $struct_name {
                $struct_name(val)
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
        ],
        $($extra: tt)*
    ) => {
        crate::define_model_type!(
            $(#[$struct_meta])*
            $vis struct $struct_name(
                $(#[$member_meta])*
                $inner_type
            ),
            [
                $(
                    $mvis ($name: $val),
                )*
            ],
            $($extra)*
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
        ],
        $($extra: tt)*
    ) => {
        crate::define_model_type!(
            $(#[$struct_meta])*
            $vis struct $struct_name(
                $(#[$member_meta])*
                $inner_type
            ),
            [
                $(
                    $mvis ($name: $val),
                )*
            ],
            $($extra)*
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
        display = true,
    ) => {
        crate::define_composed_type!(
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
            fn from_slice(mut slice: &[u8]) -> Result<(usize, Self), crate::types::FromU8Error<Self>> {
                use crate::types::FromU8Error;
                let mut ret = Self::get_none();
                let mut is_valid = true;

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

                drop(slice);

                if is_valid {
                    Ok((total, ret))
                } else {
                    Err(
                        crate::types::FromU8Error::InvalidValue((total, Some(ret)))
                    )
                }
            }

            fn to_slice(&self) -> Box<[u8]> {
                let mut ret = Vec::new();

                $(
                    match self.$member {
                        None => {
                            return ret.into_boxed_slice();
                        },
                        Some(ref s) => {
                            ret.extend_from_slice(s.to_slice().as_ref());
                        }
                    };
                )*

                ret.into_boxed_slice()
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

        impl crate::types::model::ComposedFromU8Array for $struct_name {
            fn is_some(&self) -> bool {
                $(
                    self.$member.is_some() &&
                )* true
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
