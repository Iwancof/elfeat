#![feature(split_array)]
#![feature(maybe_uninit_uninit_array)]

pub mod file;

#[macro_use]
pub mod types;

use types::Array;

define_model_type!(
    #[derive(Copy, Clone, PartialEq, Eq)]
    struct MT1(i16),
    [
        pub (VAL: 0),
    ]
);
define_model_type!(
    #[derive(Copy, Clone, PartialEq, Eq)]
    struct MT2(u128),
    pub
    [
        (VAL: 0x88),
    ]
);

define_composed_type!(
    struct Test {
        inner: Array<MT1, 2>,
    },
    display_implementation = true
);
define_composed_type!(
    struct MockType {
        a: Array<MT1, 3>,
        v: MT2,
        tmp: MT1,
        hoge: Array<Array<MT1, 2>, 2>,
        t: Test,
    },
    display_implementation = true
);

fn main() {
    let d = MockType {
        a: [MT1::VAL, MT1::VAL, MT1::VAL].into(),
        v: MT2::VAL,
        tmp: MT1::VAL,
        hoge: [[MT1::VAL, MT1(0x30)].into(), [MT1::VAL, MT1::VAL].into()].into(),
        t: Test {
            inner: [MT1(10), MT1::VAL].into(),
        },
    };

    println!("{}", d);
}
