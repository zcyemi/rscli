use nom::{IResult,HexDisplay,be_u8,le_u8,le_u16,le_u32};

use std::option::Option;

use crate::rscil::loader::util::DataInfo;
use crate::rscil::loader::util::parse_datainfo;
use crate::rscil::loader::util::parse_str_pad;
use crate::rscil::loader::util::calculate_bits_vec;

use crate::rscil::loader::winpe::{win_pe,WinPE};

#[derive(Debug)]
pub struct CLIData<'a>{
    pub header: CLIHeader,
    pub metadata:CLRMetadata<'a>,

}

impl CLIData{


    pub fn parse(i:&[u8])-> Option<(&[u8],CLIData)>{

    }
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

impl CLIHeader{


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
    pub tilde_stream: CLRTildeStream,
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
    tilde_stream: parse_tilde_stream >>
    (CLRMetadata{
        major_version:major_version,
        minor_version:minor_version,
        clr_ver_str:clr_ver_str,
        num_of_stream:num_of_stream,
        stream_header: headers,
        tilde_stream:tilde_stream,
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

#[derive(Debug)]
pub struct CLRTildeStream{
    pub major_ver: u8,
    pub minor_ver: u8,
    pub heap_size: u8,
    pub valid:Vec<u8>,
    pub sorted:Vec<u8>,
    pub rows: Vec<u32>,
}

named!(pub parse_tilde_stream<&[u8],CLRTildeStream>,do_parse!(
    take!(4) >>
    major_ver: le_u8 >>
    minor_ver: le_u8 >>
    heap_size: le_u8 >>
    tag!(&[0x01]) >>
    valid: count!(le_u8,8)>>
    sorted: count!(le_u8,8) >>
    rows: count!(le_u32,calculate_bits_vec(&valid) as usize) >>
    (
        CLRTildeStream{
            major_ver:major_ver,
            minor_ver: minor_ver,
            heap_size: heap_size,
            valid: valid,
            sorted: sorted,
            rows:rows,
        }
    )
));


//pub struct CLRAssemblyTable<'a>{
//    pub data_bulk: &'a [u8],
//    pub assembly: Vec<CLRMetaAssembly>,
//}

//impl CLRAssemblyTable{
//
//}

//pub fn parse_clr_meta_assembly(i:&[u8])-> IResult<&[u8],u8>{
//    do_parse!()
//}

pub struct  CLRMetaAssembly{
//    pub hash_alg_id: u32,
//    pub major_version: u16,
//    pub minor_version:u16,
//    pub build_number:u16,
//    pub revision_number:u16,
//    pub flags:u32,
//    pub name:u32,
//    pub culture:u32,
}



#[derive(Debug)]
pub struct CLRDll{

}




impl CLRDll{

    pub fn load(data:&[u8])->Option<CLRDll>{

        Option::None
    }


    named!(parser_dll<&[u8],DllFile>,do_parse!(
    pe: win_pe >>
    take!(16) >>
    clidata: parse_clidata >>
    (DllFile{
        pe:pe,
        cli:clidata,
    })
    ));


    fn parse_data(d:&[u8])-> Option<CLRDll>{


        let xx = parser_dll(d);




        Option::None
    }
}
