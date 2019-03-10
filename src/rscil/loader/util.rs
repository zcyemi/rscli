use nom::{IResult,HexDisplay,be_u8,le_u8,le_u16,le_u32};

use std::str;

pub type rva = u32;

#[derive(Debug)]
pub struct DataInfo {
    pub rva: rva,
    pub size: u32,
}

fn xxx(){

    let x = &"xxx";

}

named!(pub parse_datainfo<&[u8],DataInfo>,do_parse!(
    rva: le_u32 >>
    size: le_u32 >>
    (DataInfo{rva:rva,size:size})
));


named!(pub parse_str_pad<&[u8],&str>,do_parse!(
    str: take_till!(|ch| ch == 0_u8) >>
    take!((4 - (str.len()+1) % 4) % 4 + 1) >>
    (
        str::from_utf8(str).unwrap()
    )
));