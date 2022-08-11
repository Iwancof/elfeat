use crate::{define_composed_type, define_model_type};

define_model_type!(
    #[derive(PartialEq, Eq, Clone)]
    pub struct Magic(u32),
    pub
    [
        (VALID: 0x04034b50),
    ],
    display = true, bitflags = false,
);

define_model_type!(
    #[derive(PartialEq, Eq, Clone)]
    pub struct Version(u16),
    pub
    [
    ],
    display = true, bitflags = false,
);

define_model_type!(
    #[derive(PartialEq, Eq, Clone)]
    pub struct BitFlags(u16),
    pub
    [
    ],
    display = true, bitflags = true,
);

define_model_type!(
    #[derive(PartialEq, Eq, Clone)]
    pub struct Time(u16),
    pub
    [
    ],
    display = true, bitflags = false,
);

define_model_type!(
    #[derive(PartialEq, Eq, Clone)]
    pub struct Data(u16),
    pub
    [
    ],
    display = true, bitflags = false,
);

define_model_type!(
    #[derive(PartialEq, Eq, Clone)]
    pub struct CRC32(u32),
    pub
    [
    ],
    display = true, bitflags = false,
);

define_model_type!(
    #[derive(PartialEq, Eq, Clone)]
    pub struct Size(u32),
    pub
    [
    ],
    display = true, bitflags = false,
);

define_model_type!(
    #[derive(PartialEq, Eq, Clone)]
    pub struct Length(u16),
    pub
    [
    ],
    display = true, bitflags = false,
);

define_composed_type!(
    #[derive(PartialEq, Eq, Clone)]
    pub struct Header {
        magic: Option<Magic>,
        version: Option<Version>,
        general_purpose_bf: Option<BitFlags>,
        last_modify_time: Option<Time>,
        last_modify_data: Option<Data>,
        crc32: Option<CRC32>,
        compressed_size: Option<Size>,
        uncompressed_size: Option<Size>,
        file_name_length: Option<Length>,
        extra_field_length: Option<Length>,
    },
    display = true,
);
