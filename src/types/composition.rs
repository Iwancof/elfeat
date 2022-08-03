#[macro_export]
macro_rules! define_composition_vo {
    ($($struct_vis: tt)? struct $struct_name: ident {
        $($([$vis: tt])? $member: ident: $member_type: ty,)*
            // FIXME
    }) => {
        #[repr(C)]
        #[derive(Debug, Clone, PartialEq, Eq)]
        $($struct_vis)? struct $struct_name {
            $($($vis)? $member: $member_type),*
        }

        impl $struct_name {
            pub fn from_member_mut<'a>(
                $($member: &'a mut $member_type),*
            ) -> &'a mut $struct_name {
                use memoffset::offset_of;

                let ret_ptr = Self::from_member_mut_ptr($($member),*) as usize;

                $(
                    assert_eq!(
                        ret_ptr + offset_of!($struct_name, $member),
                        ($member as *mut $member_type as usize)
                    );
                )*

                // Safety: Probably ok
                let ret = unsafe { &mut *(ret_ptr as *mut Self) };

                $(
                    assert_eq!(
                        &mut ret.$member, $member
                    );
                )*

                ret
            }

            fn from_member_mut_ptr(
                $(#[allow(unused)] $member: *mut $member_type),*
            ) -> *mut Self {
                helper_get_first_tt!($($member, )*) as *mut helper_get_first_tt!($($member_type,)*) as *mut Self
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

                let return_object = $struct_name::from_member_mut($($member,)*);

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
    use super::*;
    use crate::types::elf::ElfHeader;
    use crate::types::repr_u8::{Cu8, ReprCArray};

    #[test]
    fn make_composition_ref() {
        let mut original = ElfHeader {
            e_ident: ElfMagic {
                inner: ReprCArray::new([Cu8::from_inner(0); 16]),
            },
            e_type: ElfType::ET_EXEC,
            e_machine: ElfMachine::EM_SH,
            e_version: ElfVersion::EV_NONE,
        };

        let i = &mut original.e_ident;
        let t = &mut original.e_type;
        let m = &mut original.e_machine;
        let v = &mut original.e_version;

        let r = ElfHeader::from_member_mut(i, t, m, v);

        assert_eq!(
            r.e_ident,
            ElfMagic {
                inner: ReprCArray::new([Cu8::from_inner(0); 16]),
            }
        );
        assert_eq!(r.e_type, ElfType::ET_EXEC);
        assert_eq!(r.e_machine, ElfMachine::EM_SH);
        assert_eq!(r.e_version, ElfVersion::EV_NONE);
    }
}