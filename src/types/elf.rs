use super::repr_u8::Cu16;
use crate::define_enchanted_type;

define_enchanted_type!(
    ElfType,
    Cu16,
    [NONE, Cu16::from_inner(0x0)],
    [REL, Cu16::from_inner(0x1)],
    [EXEC, Cu16::from_inner(0x2)],
    [DYN, Cu16::from_inner(0x3)],
    [CORE, Cu16::from_inner(0x4)],
    [LOOS, Cu16::from_inner(0xfe00)],
    [HIOS, Cu16::from_inner(0xfeff)],
    [LOPROC, Cu16::from_inner(0xff00)],
    [HIPROC, Cu16::from_inner(0xffff)],
);
