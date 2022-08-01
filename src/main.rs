pub mod file;

pub mod type_macro;

use file::*;

use std::fs::File;
fn main() {
    let file = File::open("./bin/main").unwrap();

    let mut bw = BinaryWrapper::from_file(file);

    let mut v = bw.interpret_at::<ElfHeader>(0).instantiate();
    let r = v.as_mut();
    println!("{:?}", r);
    r.e_type = ElfType::NONE;

    bw.write_back_obj(v);

    let mut v = bw.interpret_at::<ElfHeader>(0).instantiate();
    let r = v.as_mut();
    println!("{:?}", r);

    bw.write_back_obj(v);
}

define_prim_wrap!(
    ElfType,
    u16,
    [NONE, 0],
    [REL, 1],
    [EXEC, 2],
    [DYN, 3],
    [CORE, 4],
    [LOOS, 0xfe00],
    [HIOS, 0xfeff],
    [LOPROC, 0xff00],
    [HIPROC, 0xffff],
);

/*
#[repr(C)]
#[derive(Clone, Copy)]
struct ElfType {
    inner: u16,
}

impl ElfType {
    const NONE: u16 = 0;
    const REL: u16 = 1;
    const EXEC: u16 = 2;
    const DYN: u16 = 3;
    const CORE: u16 = 4;
    const LOOS: u16 = 0xfe00;
    const HIOS: u16 = 0xfeff;
    const LOPROC: u16 = 0xff00;
    const HIPROC: u16 = 0xffff;
}

impl core::fmt::Display for ElfType {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let s = match self.inner {
            Self::NONE => "None",
            Self::REL => "Rel",
            Self::EXEC => "Exec",
            Self::DYN => "Dyn",
            Self::CORE => "Core",
            Self::LOOS => "Loos",
            Self::HIOS => "Hios",
            Self::LOPROC => "Loproc",
            Self::HIPROC => "Hiproc",
            _ => "Unknown",
        };

        write!(fmt, "{}", s)
    }
}

impl core::fmt::Debug for ElfType {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(fmt, "{}", self.inner)
    }
}
*/

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct ElfHeader {
    // this struct will move
    e_ident: [u8; 16],
    e_type: ElfType,
    e_machine: u16,
    e_version: u32,
    e_entry: u32,
    shoff: u32,
}
