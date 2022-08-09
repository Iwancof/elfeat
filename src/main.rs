#![feature(split_array)]
#![feature(maybe_uninit_uninit_array)]

pub mod file;
use file::Sequential;

#[macro_use]
pub mod types;

use types::elf::ElfHeader;

fn main() {
    use std::fs::File;
    use std::io::Read;
    let mut f = File::open("./bin/main").unwrap();
    let mut v = vec![];

    f.read_to_end(&mut v).unwrap();

    let s = Sequential::from_vec(v);

    let mut r1 = s.to_seeakble();
    let (_read, header): (usize, ElfHeader) = r1.interpret_abs_pos(0).to_tuple().1.unwrap();

    println!("{}", header);
}
