#![feature(split_array)]
#![feature(maybe_uninit_uninit_array)]

pub mod file;
use file::Sequential;

#[macro_use]
pub mod types;

use types::elf::ElfHeader;

use crate::{file::InterpretObject, types::Array};

fn main() {
    use std::fs::File;
    use std::io::Read;
    let mut f = File::open("./bin/main").unwrap();
    let mut v = vec![];

    f.read_to_end(&mut v).unwrap();

    let s = Sequential::from_vec(v);

    let seeker = s.to_seeakble();
    let header: ElfHeader = seeker.interpret_abs_pos(0).to_tuple_unwrap().1;
    let entry_seeker = *header.get_e_entry_unwrap();

    let mut entry_seek = seeker.clone().seek(entry_seeker.into());
    println!(
        "{:x}",
        entry_seek
            .interpret_next::<Array<u8, 16>>()
            .to_tuple_unwrap()
            .1
    );
}
