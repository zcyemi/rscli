use nom::{IResult,HexDisplay,be_u8,le_u8,le_u16,le_u32};

use crate::rscil::loader::util::DataInfo;
use crate::rscil::loader::util::parse_datainfo;
use crate::rscil::loader::util::parse_str_pad;

#[derive(Debug)]
pub struct CLIData<'a>{
    pub header: CLIHeader,
    pub metadata:CLRMetadata<'a>,

}

// total bytes 8 + 72
named!(pub parse_clidata<&[u8],CLIData>,do_parse!(
    take!(8) >>
    header: parse_cliheader >>
    metadata: parse_clr_metadata >>
    (CLIData{
        header:header,
        metadata:metadata,
    })
));

#[derive(Debug)]
pub struct CLIHeader{
    pub major_runtime_ver: u16,
    pub minor_runtime_ver: u16,
    pub metadata: DataInfo,
    pub flags: u32,
    pub entry_point_token: u32,
    pub strong_name_signature: DataInfo,
}

// total bytes 8 + 72
named!(pub parse_cliheader<&[u8],CLIHeader>,do_parse!(
    tag!(&[0x48,0,0,0]) >>
    maj_runtime_ver: le_u16 >>
    min_runtime_ver: le_u16 >>
    metadata: parse_datainfo >>
    flags: le_u32 >>
    entry_point_token: le_u32 >>
    resources: parse_datainfo >>
    strong_name_signature: parse_datainfo >>
    code_manager_tbl: take!(8) >>
    vtable_fixups: parse_datainfo >>
    export_addr_tbl_jumps: take!(8) >>
    managed_native_header: take!(8) >>
    (CLIHeader{
        major_runtime_ver: maj_runtime_ver,
        minor_runtime_ver: min_runtime_ver,
        metadata: metadata,
        flags:flags,
        entry_point_token:entry_point_token,
        strong_name_signature: strong_name_signature,
    })
));


#[derive(Debug)]
pub struct CLRMetadata<'a>{
    pub major_version: u16,
    pub minor_version: u16,
    pub clr_ver_str:&'a str,
    pub num_of_stream: u16,

    pub stream_header: Vec<CLRStreamHeader<'a>>,
}

named!(pub parse_clr_metadata<&[u8],CLRMetadata>,do_parse!(
    take_until!(&[0x42,0x53,0x4A,0x42][..]) >>
    tag!(&[0x42,0x53,0x4A,0x42]) >>
    major_version: le_u16 >>
    minor_version: le_u16 >>
    take!(4) >>
    clr_ver_sz: le_u32 >>
    clr_ver_str: take_str!(clr_ver_sz) >>
    take!(2) >>
    num_of_stream: le_u16 >>
    headers: many_m_n!(num_of_stream as usize,num_of_stream as usize,parse_clr_stream_header) >>
    (CLRMetadata{
        major_version:major_version,
        minor_version:minor_version,
        clr_ver_str:clr_ver_str,
        num_of_stream:num_of_stream,
        stream_header: headers,
    })
));



#[derive(Debug)]
pub struct CLRStreamHeader<'a>{
    pub offset: u32,
    pub size: u32,
    pub name:&'a str,
}

named!(parse_clr_stream_header<&[u8],CLRStreamHeader>,do_parse!(
    offset: le_u32 >>
    size: le_u32 >>
    name: parse_str_pad >>
    (CLRStreamHeader{
        offset: offset,
        size: size,
        name: name
    })
));

