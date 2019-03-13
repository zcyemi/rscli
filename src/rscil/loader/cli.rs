use nom::{IResult,HexDisplay,be_u8,le_u8,le_u16,le_u32};

use crate::rscil::loader::util::DataInfo;
use crate::rscil::loader::util::parse_str_pad;
use crate::rscil::loader::util::calculate_bits_vec;

#[derive(Debug)]
pub struct CLIData<'a>{
    pub header: CLIHeader,
    pub metadata:CLIMetadata<'a>,

}

impl<'a> CLIData<'a>{
    // total bytes 8 + 72
    named!(pub parse<&[u8],CLIData>,do_parse!(
        take!(8) >>
        header: call!(CLIHeader::parse) >>
        metadata: call!(CLIMetadata::parse) >>
        (CLIData{
            header:header,
            metadata:metadata,
        })
    ));
}


#[derive(Debug)]
pub struct CLIHeader{
    pub major_runtime_ver: u16,
    pub minor_runtime_ver: u16,
    pub metadata: DataInfo,
    pub flags: u32,
    pub entry_point_token: u32,
    pub strong_name_signature: DataInfo,
}

impl CLIHeader{
    // total bytes 8 + 72
    named!(pub parse<&[u8],CLIHeader>,do_parse!(
        tag!(&[0x48,0,0,0]) >>
        maj_runtime_ver: le_u16 >>
        min_runtime_ver: le_u16 >>
        metadata: call!(DataInfo::parse) >>
        flags: le_u32 >>
        entry_point_token: le_u32 >>
        resources: call!(DataInfo::parse) >>
        strong_name_signature: call!(DataInfo::parse) >>
        code_manager_tbl: take!(8) >>
        vtable_fixups: call!(DataInfo::parse) >>
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
}

#[derive(Debug)]
pub struct CLIMetadata<'a>{
    pub major_version: u16,
    pub minor_version: u16,
    pub cli_ver_str:&'a str,
    pub num_of_stream: u16,
    pub stream_header: Vec<CLIStreamHeader<'a>>,
    pub tilde_stream: CLITildeStream,
}

impl<'a> CLIMetadata<'a>{

    named!(pub parse<&[u8],CLIMetadata>,do_parse!(
        take_until!(&[0x42,0x53,0x4A,0x42][..]) >>
        tag!(&[0x42,0x53,0x4A,0x42]) >>
        major_version: le_u16 >>
        minor_version: le_u16 >>
        take!(4) >>
        cli_ver_sz: le_u32 >>
        cli_ver_str: take_str!(cli_ver_sz) >>
        take!(2) >>
        num_of_stream: le_u16 >>
        headers: many_m_n!(num_of_stream as usize,num_of_stream as usize,call!(CLIStreamHeader::parse)) >>
        tilde_stream: parse_tilde_stream >>
        (CLIMetadata{
            major_version:major_version,
            minor_version:minor_version,
            cli_ver_str:cli_ver_str,
            num_of_stream:num_of_stream,
            stream_header: headers,
            tilde_stream:tilde_stream,
        })
    ));
}


#[derive(Debug)]
pub struct CLIStreamHeader<'a>{
    pub offset: u32,
    pub size: u32,
    pub name:&'a str,
}

impl<'a> CLIStreamHeader<'a>{

    named!(parse<&[u8],CLIStreamHeader>,do_parse!(
        offset: le_u32 >>
        size: le_u32 >>
        name: parse_str_pad >>
        (CLIStreamHeader{
            offset: offset,
            size: size,
            name: name
        })
    ));
}


#[derive(Debug)]
pub struct CLITildeStream{
    pub major_ver: u8,
    pub minor_ver: u8,
    pub heap_size: u8,
    pub valid:Vec<u8>,
    pub sorted:Vec<u8>,
    pub rows: Vec<u32>,
}

impl CLITildeStream{

}

named!(pub parse_tilde_stream<&[u8],CLITildeStream>,do_parse!(
    take!(4) >>
    major_ver: le_u8 >>
    minor_ver: le_u8 >>
    heap_size: le_u8 >>
    tag!(&[0x01]) >>
    valid: count!(le_u8,8)>>
    sorted: count!(le_u8,8) >>
    rows: count!(le_u32,calculate_bits_vec(&valid) as usize) >>
    (
        CLITildeStream{
            major_ver:major_ver,
            minor_ver: minor_ver,
            heap_size: heap_size,
            valid: valid,
            sorted: sorted,
            rows:rows,
        }
    )
));



