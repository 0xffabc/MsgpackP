pub(crate) struct Families;

impl Families {
    pub const POSITIVE_FIXINT: u8 = 0x00;
    pub const FIXMAP: u8 = 0x80;
    pub const FIXARRAY: u8 = 0x90;
    pub const FIXSTR: u8 = 0xa0;
    pub const NIL: u8 = 0xc0;
    pub const FALSE: u8 = 0xc2;
    pub const TRUE: u8 = 0xc3;
    pub const BIN8: u8 = 0xc4;
    pub const BIN16: u8 = 0xc5;
    pub const BIN32: u8 = 0xc6;
    pub const EXT8: u8 = 0xc7;
    pub const EXT16: u8 = 0xc8;
    pub const EXT32: u8 = 0xc9;
    pub const FLOAT32: u8 = 0xca;
    pub const FLOAT64: u8 = 0xcb;
    pub const UINT8: u8 = 0xcc;
    pub const UINT16: u8 = 0xcd;
    pub const UINT32: u8 = 0xce;
    pub const UINT64: u8 = 0xcf;
    pub const INT8: u8 = 0xd0;
    pub const INT16: u8 = 0xd1;
    pub const INT32: u8 = 0xd2;
    pub const INT64: u8 = 0xd3;
}
