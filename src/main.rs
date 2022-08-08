#![feature(split_array)]
#![feature(maybe_uninit_uninit_array)]

pub mod file;
pub mod types;

use std::fs::File;

fn main() {
    /*
    let file = File::open("./bin/main").unwrap();

    let mut bw = BinaryWrapper::from_file(file);
    let mut slice = bw.to_mut_slice();

    let (_h, v, _) = slice
        .interpret_relat_mut::<ElfHeader>(0)
        .ignore_validity()
        .unwrap();

    println!("{}", v);

    let mut file = File::create("./bin/out").unwrap();
    use std::io::Write;
    file.write_all(bw.binary.as_ref()).unwrap();
    */
}
