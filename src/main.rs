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

    let section_header_offset = *header.e_shoff.unwrap().inner();
    let section_strtab_index = *header.e_shstrndx.unwrap().inner() as usize;

    let mut section_seeker = s.to_seeakble().seek(section_header_offset);

    let mut section_headers = vec![];
    for _i in 0.. {
        let (_at, read) = section_seeker
            .interpret_next::<section_header::Header>()
            .to_tuple();

        if read.is_err() {
            break;
        }

        section_headers.push(read.unwrap().1);
    }

    let strtab = section_headers[section_strtab_index];
    println!("{}", strtab);

    let mut sh_name_seeker = s.to_seeakble().seek(*strtab.get_sh_offset_unwrap().inner());
    let mut section_names = vec![];

    // TODO: range limited seeker

    let mut read = 0;
    for _i in 0.. {
        let (_pos, s) = sh_name_seeker
            .interpret_next::<types::primitive::NullTermString>()
            .to_tuple();

        let (size, s) = s.unwrap();
        read += size;

        if *strtab.get_sh_size_unwrap().inner() < read as _ {
            break;
        }

        println!("{}", s);
        section_names.push(s);
    }

    println!("{:?}", section_names);

    for sh in section_headers {
        let (_, name) = section_seeker
            .interpret_abs_pos::<crate::types::primitive::NullTermString>(
                *sh.get_sh_name_unwrap().inner() as usize + *strtab.get_sh_offset_unwrap().inner(),
            )
            .to_tuple_unwrap();
        println!("name = {}", name);
        println!("{}", sh);
    }
}
