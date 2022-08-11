use elfeat::{file, types};

use file::Sequential;
use std::collections::HashMap;

use types::elf::*;

fn main() {
    use std::fs::File;
    use std::io::Read;
    let mut f = File::open("./bin/static").unwrap();
    let mut v = vec![];

    f.read_to_end(&mut v).unwrap();

    let s = Sequential::from_vec(v);

    let mut main_seeker = s.to_seeakble();
    let header: elf_header::Header = main_seeker.interpret_abs_pos(0).to_tuple_unwrap().1;

    println!("{}", header);

    if header.e_shstrndx.unwrap().is_SHN_UNDEF() {
        panic!();
    }

    let section_header_offset = header.e_shoff.unwrap().inner();
    let section_strtab_index = header.e_shstrndx.unwrap().inner() as usize;

    main_seeker.seek(section_header_offset);

    let mut section_headers = vec![];
    for _i in 0.. {
        let (_at, read) = main_seeker
            .interpret_next::<section_header::Header>()
            .to_tuple();

        if read.is_err() {
            break;
        }

        section_headers.push(read.unwrap().1);
    }

    let strtab = section_headers[section_strtab_index];
    println!("{}", strtab);

    let mut sections = HashMap::new();

    // TODO: range limited seeker

    for sh in section_headers {
        let (_, name) = main_seeker
            .interpret_abs_pos::<types::primitive::NullTermString>(
                sh.get_sh_name_unwrap().inner() as usize + strtab.get_sh_offset_unwrap().inner(),
            )
            .to_tuple_unwrap();

        sections.insert(name, sh);
    }

    let interp = sections[".plt"];
    println!("{}", interp);

    main_seeker.seek(interp.get_sh_offset_unwrap().inner());

    let up = main_seeker.interpret_next::<u32>().to_tuple_unwrap().1;
    let down = main_seeker.interpret_next::<u32>().to_tuple_unwrap().1;

    println!("{:x}", ((up as u64) << 32) + down as u64);
}
