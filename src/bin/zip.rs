use elfeat::{file, types};

use file::Sequential;
use types::zip::*;

use std::{fs::File, io::Read};

fn main() {
    let mut f = File::open("./example_bin/archives.zip").unwrap();
    let mut v = vec![];
    f.read_to_end(&mut v).unwrap();

    let s = Sequential::from_vec(v);

    let mut main_seeker = s.to_seeakble();
    let header = main_seeker.interpret_next::<Header>().to_tuple_unwrap().1;

    println!("{}", header);
}
