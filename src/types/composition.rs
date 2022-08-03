use super::elf::{ElfMachine, ElfMagic, ElfType, ElfVersion};

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElfHeader {
    pub e_ident: ElfMagic,
    pub e_type: ElfType,
    pub e_machine: ElfMachine,
    pub e_version: ElfVersion,
}

impl ElfHeader {
    pub fn from_member_mut<'a>(
        e_ident: &'a mut ElfMagic,
        e_type: &'a mut ElfType,
        e_machine: &'a mut ElfMachine,
        e_version: &'a mut ElfVersion,
    ) -> &'a mut Self {
        use memoffset::offset_of;

        let ret_ptr = Self::from_member_mut_ptr(e_ident, e_type, e_machine, e_version) as usize;

        assert_eq!(
            ret_ptr + offset_of!(Self, e_ident),
            (e_ident as *mut _ as usize)
        );
        assert_eq!(
            ret_ptr + offset_of!(Self, e_type),
            (e_type as *mut _ as usize)
        );
        assert_eq!(
            ret_ptr + offset_of!(Self, e_machine),
            (e_machine as *mut _ as usize)
        );
        assert_eq!(
            ret_ptr + offset_of!(Self, e_version),
            (e_version as *mut _ as usize)
        );

        // Safety: Probably ok
        let ret = unsafe { &mut *(ret_ptr as *mut Self) };

        assert_eq!(&mut ret.e_ident, e_ident);
        assert_eq!(&mut ret.e_type, e_type);
        assert_eq!(&mut ret.e_machine, e_machine);
        assert_eq!(&mut ret.e_version, e_version);

        ret
    }

    fn from_member_mut_ptr<'a>(
        #[allow(unused)] e_ident: *mut ElfMagic,
        #[allow(unused)] e_type: *mut ElfType,
        #[allow(unused)] e_machine: *mut ElfMachine,
        #[allow(unused)] e_version: *mut ElfVersion,
    ) -> *mut Self {
        e_ident as *mut ElfType as *mut Self
    }
}

impl crate::types::repr_u8::VOWrapU8Array for ElfHeader {
    fn is_sanity(&self) -> bool {
        self.e_ident.is_sanity()
            && self.e_type.is_sanity()
            && self.e_machine.is_sanity()
            && self.e_version.is_sanity()
    }
}

impl crate::types::repr_u8::VOWrapU8ArrayMut for ElfHeader {
    fn constitude_mut(remain: &mut [u8]) -> crate::types::repr_u8::VOConstitudeResultMut<'_, Self> {
        use crate::types::repr_u8::VOConstitudeResultMut;
        let (e_ident, remain) = match ElfMagic::constitude_mut(remain) {
            VOConstitudeResultMut::Valid(v, r) => (v, r),
            VOConstitudeResultMut::Invalid(v, r) => (v, r),
            VOConstitudeResultMut::Error(e) => return VOConstitudeResultMut::Error(e),
        };
        let (e_type, remain) = match ElfType::constitude_mut(remain) {
            VOConstitudeResultMut::Valid(v, r) => (v, r),
            VOConstitudeResultMut::Invalid(v, r) => (v, r),
            VOConstitudeResultMut::Error(e) => return VOConstitudeResultMut::Error(e),
        };
        let (e_machine, remain) = match ElfMachine::constitude_mut(remain) {
            VOConstitudeResultMut::Valid(v, r) => (v, r),
            VOConstitudeResultMut::Invalid(v, r) => (v, r),
            VOConstitudeResultMut::Error(e) => return VOConstitudeResultMut::Error(e),
        };
        let (e_version, remain) = match ElfVersion::constitude_mut(remain) {
            VOConstitudeResultMut::Valid(v, r) => (v, r),
            VOConstitudeResultMut::Invalid(v, r) => (v, r),
            VOConstitudeResultMut::Error(e) => return VOConstitudeResultMut::Error(e),
        };

        let ret_obj = ElfHeader::from_member_mut(e_ident, e_type, e_machine, e_version);

        use crate::types::repr_u8::VOWrapU8Array;
        if ret_obj.is_sanity() {
            VOConstitudeResultMut::Valid(ret_obj, remain)
        } else {
            VOConstitudeResultMut::Invalid(ret_obj, remain)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
