#![feature(split_array)]
#![feature(maybe_uninit_uninit_array)]
pub mod file;
use file::Sequential;

#[macro_use]
pub mod types;

use types::elf::*;
use types::model::ComposedFromU8Array;

use crate::{file::InterpretObject, types::Array};

fn main() {
    use std::fs::File;
    use std::io::Read;
    let mut f = File::open("./bin/main").unwrap();
    let mut v = vec![];

    f.read_to_end(&mut v).unwrap();

    let s = Sequential::from_vec(v);

    let main_seeker = s.to_seeakble();
    let header: elf_header::Header = main_seeker.interpret_abs_pos(0).to_tuple_unwrap().1;

    println!("{}", header);

    if header.e_shstrndx.unwrap().is_SHN_UNDEF() {
        panic!();
    }

    let section_header_offset = header.e_shoff.unwrap().inner();

    let mut section_seeker = s.to_seeakble().seek(section_header_offset);

    let strtab_header: section_header::Header = loop {
        let (_at, read) = section_seeker
            .interpret_next::<section_header::Header>()
            .to_tuple_unwrap();

        if read.get_sh_type_unwrap().is_SHT_STRTAB() && read.get_sh_flags_unwrap().is_SHF_ALLOC() {
            break read;
        }
    };

    println!("{}", strtab_header);

    section_seeker.seek(strtab_header.get_sh_offset_unwrap().inner());

    for _i in 0..20 {
        let r = section_seeker
            .interpret_next::<types::primitive::NullTermString>()
            .to_tuple_unwrap();
        println!("{}", r.1);
    }
}
