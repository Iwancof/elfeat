#![feature(split_array)]
#![feature(maybe_uninit_uninit_array)]
pub mod file;
use file::Sequential;

#[macro_use]
pub mod types;

use types::elf::ElfHeader;
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
    let header: ElfHeader = main_seeker.interpret_abs_pos(0).to_tuple_unwrap().1;

    println!("{}", header);

    if header.e_shstrndx.unwrap().is_SHN_UNDEF() {
        panic!();
    }

    let mut section_header_offset = header.e_shoff.unwrap().inner();
    section_header_offset += (64 + 16 + 16) * 0;

    let mut section_seeker = s.to_seeakble().seek(section_header_offset);

    let read = section_seeker
        .interpret_next::<Array<u8, 100>>()
        .to_tuple_unwrap();

    println!("{:?}", read);
}
