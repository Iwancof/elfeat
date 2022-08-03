#![feature(const_slice_from_raw_parts_mut)]
#![feature(const_mut_refs)]

extern crate libc;

pub mod file;
pub mod types;

use file::*;
use types::composition::ElfHeader;

use std::fs::File;

fn main() {
    let file = File::open("./bin/main").unwrap();

    let mut bw = BinaryWrapper::from_file(file);
    let mut slice = bw.to_mut_slice();

    let (v, _) = slice
        .interpret_next_mut::<ElfHeader>()
        .unwrap_no_head_validity();

    println!("{:?}", v);
}
