#![feature(split_array)]
#![feature(maybe_uninit_uninit_array)]

pub mod file;
use file::{BinaryReader, Sequential};

#[macro_use]
pub mod types;

use types::Array;

fn main() {
    let s = Sequential::from_vec(vec![1, 2, 3]);

    let r1 = s.to_seeakble();
    let r2 = s.to_seeakble();

    println!("{:?}", r1);
    println!("{:?}", r2);
}
