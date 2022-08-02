pub mod file;

extern crate libc;

pub mod types;

use file::*;

use std::fs::File;
fn main() {
    let file = File::open("./bin/main").unwrap();

    let mut bw = BinaryWrapper::from_file(file);

    /*
    let mut v = bw.interpret_at::<ElfHeader>(0).instantiate();
    let r = v.as_mut();
    println!("{:?}", r);
    r.e_type = ElfType::NONE;

    bw.write_back_obj(v);

    let mut v = bw.interpret_at::<ElfHeader>(0).instantiate();
    let r = v.as_mut();
    println!("{:?}", r);

    bw.write_back_obj(v);
    */
}

/*

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
*/
