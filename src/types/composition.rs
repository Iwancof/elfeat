#[macro_export]
macro_rules! helper_get_first_tt {
    ($first: tt, $($e: tt,)*) => {
        $first
    };
}

#[macro_export(local_inner_macros)]
macro_rules! define_composition_vo {
    ($([$struct_vis: tt])? struct $struct_name: ident {
        $($([$vis: tt])? $member: ident: $member_type: ty,)*
            // FIXME
    }) => {
        #[repr(C)]
        #[repr(align(4))]
        #[derive(Debug, Clone, PartialEq, Eq)]
        $($struct_vis)? struct $struct_name {
            $($($vis)? $member: $member_type),*
        }

        impl $struct_name {
            #[allow(unused)]
            pub fn from_member_mut<'a>(
                $($member: &'a mut $member_type),*
            ) -> Option<&'a mut $struct_name> {
                paste::paste! {
                    let ret_ptr = Self::from_member_mut_ptr($($member),*);
                    let align = core::mem::align_of::<$struct_name>();

                    if !ret_ptr.is_aligned_to(align) {
                        // return None;
                    };

                    let ret_ptr = ret_ptr as usize;

                    $(
                        if ret_ptr + memoffset::offset_of!($struct_name, $member) != ($member as *mut $member_type as usize) {
                            return None;
                        }
                    )*

                    $(
                            let [<address_of_ $member>]: usize = $member as *mut $member_type as usize; // to address value.
                            drop($member); // reference is dropped.
                    )*

                    // Safety: Probably ok
                    // memory range of $member(s) has 0 &mut reference. and align is ok. so, creating &mut Self is
                    // valid.
                    // And, by strict aliasing rules we can only treat $member as $member_type, and
                    // Self's layout is same. so, this is not undefined behavior.   ... I think.
                    let ret = unsafe { &mut *(ret_ptr as *mut Self) };

                    $(
                        core::debug_assert_eq!(
                            &mut ret.$member as *mut $member_type as usize, [<address_of_ $member>]
                        );
                    )*

                    return Some(ret);
                }
            }

            #[allow(unused)]
            fn from_member_mut_ptr(
                $(#[allow(unused)] $member: *mut $member_type),*
            ) -> *mut Self {
                helper_get_first_tt!($($member, )*) as *mut helper_get_first_tt!($($member_type,)*) as *mut Self
            }

            #[allow(unused)]
            pub fn from_member_ref<'a>(
                $($member: &'a $member_type),*
            ) -> Option<&'a $struct_name> {
                paste::paste!{
                    let ret_ptr = Self::from_member_ptr($($member),*);
                    let align = core::mem::align_of::<$struct_name>();

                    if !ret_ptr.is_aligned_to(align) {
                        return None;
                    };

                    let ret_ptr = ret_ptr as usize;

                    $(
                        if ret_ptr + memoffset::offset_of!($struct_name, $member) != ($member as *const $member_type as usize) {
                            return None;
                        }
                    )*

                    $(
                        let [<address_of_ $member>]: usize = $member as *const $member_type as usize;
                        drop($member);
                    )*

                    // Safety: Probably ok
                    // memory range of $member(s) has 0 &mut reference. and align is ok. so, creating &mut Self is
                    // valid.
                    // And, by strict aliasing rules we can only treat $member as $member_type, and
                    // Self's layout is same. so, this is not undefined behavior.   ... I think.
                    let ret = unsafe { &*(ret_ptr as *const Self) };

                    $(
                        // It can be replaced with debug_assert_eq.
                        core::debug_assert_eq!(
                            &ret.$member as *const $member_type as usize, [<address_of_ $member>]
                        );
                    )*

                    Some(ret)
                }
            }

            #[allow(unused)]
            fn from_member_ptr(
                $(#[allow(unused)] $member: *const $member_type),*
            ) -> *const Self {
                helper_get_first_tt!($($member, )*) as *const helper_get_first_tt!($($member_type,)*) as *const Self
            }
        }

        impl crate::types::repr_u8::VOWrapU8Array for $struct_name {
            fn is_sanity(&self) -> bool {
                $(self.$member.is_sanity() &&)* true
            }
        }

        impl crate::types::repr_u8::VOWrapU8ArrayMut for $struct_name {
            fn constitude_mut(remain: &mut [u8]) -> crate::types::repr_u8::VOConstitudeResultMut<'_, $struct_name> {
                use crate::types::repr_u8::VOConstitudeResultMut;

                $(
                    let ($member, remain) = match <$member_type>::constitude_mut(remain) {
                        VOConstitudeResultMut::Valid(v, r) => (v, r),
                        VOConstitudeResultMut::Invalid(v, r) => (v, r),
                        VOConstitudeResultMut::Error(e) => return VOConstitudeResultMut::Error(e),
                    };
                )*

                let return_object = $struct_name::from_member_mut($($member,)*).unwrap();

                use crate::types::repr_u8::VOWrapU8Array;
                if return_object.is_sanity() {
                    VOConstitudeResultMut::Valid(return_object, remain)
                } else {
                    VOConstitudeResultMut::Invalid(return_object, remain)
                }
            }
        }

    };
}

#[cfg(test)]
mod tests {
    use crate::types::elf::*;

    define_composition_vo!(
        struct MockComposition {
            e_type: ElfType,
            e_machine: ElfMachine,
            e_version: ElfVersion,
        }
    );

    #[test]
    fn make_composition_ref() {
        let mut original = MockComposition {
            e_type: ElfType::ET_EXEC,
            e_machine: ElfMachine::EM_SH,
            e_version: ElfVersion::EV_NONE,
        };

        let t = &mut original.e_type;
        let m = &mut original.e_machine;
        let v = &mut original.e_version;

        let r = MockComposition::from_member_mut(t, m, v).unwrap();

        assert_eq!(r.e_type, ElfType::ET_EXEC);
        assert_eq!(r.e_machine, ElfMachine::EM_SH);
        assert_eq!(r.e_version, ElfVersion::EV_NONE);
    }

    #[test]
    #[should_panic]
    fn mismatch_reference() {
        let v = Box::leak(Box::new(ElfVersion::EV_NONE));
        let m = Box::leak(Box::new(ElfMachine::EM_SH));
        let t = Box::leak(Box::new(ElfType::ET_EXEC));

        let _r = MockComposition::from_member_mut(t, m, v).unwrap();
    }

    #[test]
    fn test_strict_aliasing() {
        let mut original = MockComposition {
            e_type: ElfType::ET_EXEC,
            e_machine: ElfMachine::EM_SH,
            e_version: ElfVersion::EV_NONE,
        };

        let t = &mut original.e_type;
        let m = &mut original.e_machine;
        let v = &mut original.e_version;

        let r = MockComposition::from_member_mut(t, m, v).unwrap();

        let composed_address = r as *mut MockComposition as *mut u8;
        let original_address = t as *mut ElfType as *mut u8;

        assert_eq!(original_address as usize, composed_address as usize);
        assert_eq!(original_address as usize - composed_address as usize, 0);
    }

    #[test]
    fn align_panic() {
        println!("{}", core::mem::align_of::<MockComposition>());
        println!("{}", core::mem::align_of::<ElfType>());

        panic!();
    }
}
