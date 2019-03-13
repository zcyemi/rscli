use nom::{IResult,HexDisplay,be_u8,le_u8,le_u16,le_u32};

use std::option::Option;

use crate::rscil::loader::util::DataInfo;

#[derive(Debug)]
pub struct DosHeader{}

#[derive(Debug)]
pub struct DosStub{}

#[derive(Debug)]
pub struct WinPE<'a>{
    pub coff_header:CoffHeader,
    pub coff_fields:CoffFields,
    pub nt_fields: PeNtFields,
    pub data_directories: DataDirectories,
    pub text_section: Section<'a>,
    pub rsrc_section: Section<'a>,
    pub reloc_section: Section<'a>,
}

impl WinPE{
    pub fn parse(i:&[u8])->Option<(&[u8],WinPE)>{
        let ret = match parse_win_pe(i) {
            Some(v)=>{v},
            Err(e)=>{
                println!("{}",e);
                Option::None
            }
        };
        ret
    }
}

named!(pub parse_win_pe<&[u8],WinPE>,do_parse!(
    dos_header >>
    dos_stub >>
    coffheader: coff_header >>
    coff_fields:coff_fields >>
    pe_nt_fields: pe_nt_fields >>
    data_directories: data_directories >>
    text_section: parse_section >>
    rsrc_section: parse_section >>
    reloc_section: parse_section >>
    take!(16) >>
    (WinPE{
        coff_header: coffheader,
        coff_fields: coff_fields,
        nt_fields: pe_nt_fields,
        data_directories: data_directories,
        text_section: text_section,
        rsrc_section: rsrc_section,
        reloc_section: reloc_section,
    })
));

named!(dos_header<&[u8],DosHeader>,
    do_parse!(
        tag!(&[0x4D,0x5A]) >>
        take!(62) >>
    (DosHeader{})
));

named!(dos_stub<&[u8],DosStub>,do_parse!(take!(64) >>(DosStub{})));


type rva = u32;

#[derive(Debug)]
pub struct CoffHeader{
    pub machine: u16,
    pub num_section: u16,
    pub timedate_stamp: u32,
    pointer_sbl_tbl: u32,
    num_sbl_tbl: u32,
    pub sz_opt_header: u16,
    pub characteristics:u16,
}

named!(coff_header<&[u8],CoffHeader>,do_parse!(
    tag!([0x50,0x45,0,0]) >>
    machine: le_u16 >>
    num_section: le_u16 >>
    timedate_stamp: le_u32 >>
    pointer_sbl_tbl: le_u32 >>
    num_sbl_tbl: le_u32 >>
    sz_opt_header: le_u16 >>
    characteristics: le_u16 >>
    (CoffHeader{
        machine: machine,
        num_section:num_section,
        timedate_stamp: timedate_stamp,
        pointer_sbl_tbl:pointer_sbl_tbl,
        num_sbl_tbl:num_sbl_tbl,
        sz_opt_header:sz_opt_header,
        characteristics
    })
));

#[derive(Debug)]
pub struct CoffFields{
    pub maj_linker_ver: u8,
    pub min_linker_ver:u8,
    pub size_code: u32,
    pub size_initialized_data:u32,
    pub size_uninitialized_data:u32,
    pub addr_entry_point: rva,
    pub base_of_code: rva,
    pub base_of_data: rva,
}

named!(coff_fields<&[u8],CoffFields>,do_parse!(
    tag!([0x0B,0x01]) >>
    maj_linker_ver: le_u8 >>
    min_linker_ver: le_u8 >>
    size_code: le_u32 >>
    size_initialized_data:le_u32 >>
    size_uninitialized_data:le_u32 >>
    addr_entry_point:le_u32 >>
    base_of_code: le_u32 >>
    base_of_data: le_u32 >>
    (CoffFields{
        maj_linker_ver:maj_linker_ver,
        min_linker_ver: min_linker_ver,
        size_code:size_code,
        size_initialized_data:size_initialized_data,
        size_uninitialized_data:size_uninitialized_data,
        addr_entry_point:addr_entry_point,
        base_of_code:base_of_code,
        base_of_data:base_of_data,
    })
));


#[derive(Debug)]
pub struct PeNtFields{
    //Total size 68 byte
}

named!(pe_nt_fields<&[u8],PeNtFields>,do_parse!(
    take!(68) >>
    (PeNtFields{})
));


#[derive(Debug)]
pub struct DataDirectories{
    pub export_tbl: DataInfo,
    pub import_tbl: DataInfo,
    pub resource_tbl: DataInfo,
    pub exception_tbl:DataInfo,
    pub certificate_tbl:DataInfo,
    pub base_relocation_tbl: DataInfo,
    pub debug: DataInfo,
    pub architecture_data: DataInfo,
    pub global_ptr: rva,
    pub tls_tbl: DataInfo,
    pub load_config_tbl:DataInfo,
    pub bound_import: DataInfo,
    pub import_addr_tbl: DataInfo,
    pub delay_import_descriptor: DataInfo,
    pub clr_runtime_helper: DataInfo,
}

named!(parse_datainfo<&[u8],DataInfo>,do_parse!(
    rva: le_u32 >>
    size: le_u32 >>
    (DataInfo{rva:rva,size:size})
));

named!(data_directories<&[u8],DataDirectories>,do_parse!(
    export_tbl: parse_datainfo >>
    import_tbl: parse_datainfo >>
    resource_tbl: parse_datainfo >>
    exception_tbl: parse_datainfo >>
    certificate_tbl: parse_datainfo >>
    base_relocation_tbl: parse_datainfo >>
    debug: parse_datainfo >>
    architecture_data: parse_datainfo >>
    global_ptr: le_u32 >>
    take!(4) >>
    tls_tbl: parse_datainfo >>
    load_config_tbl: parse_datainfo >>
    bound_import: parse_datainfo >>
    import_addr_tbl: parse_datainfo >>
    delay_import_descriptor: parse_datainfo >>
    clr_runtime_helper: parse_datainfo >>
    take!(8) >>
    (DataDirectories{
        export_tbl:export_tbl,
        import_tbl:import_tbl,
        resource_tbl:resource_tbl,
        exception_tbl:exception_tbl,
        certificate_tbl:certificate_tbl,
        base_relocation_tbl:base_relocation_tbl,
        debug:debug,
        architecture_data:architecture_data,
        global_ptr:global_ptr,
        tls_tbl:tls_tbl,
        load_config_tbl: load_config_tbl,
        bound_import: bound_import,
        import_addr_tbl: import_addr_tbl,
        delay_import_descriptor: delay_import_descriptor,
        clr_runtime_helper: clr_runtime_helper,
    })
));

#[derive(Debug)]
pub struct Section<'a>{
    pub name: &'a str,
    pub virtual_size:u32,
    pub virtual_addr:rva,
    pub size_of_raw_data: u32,
    pub pointer_to_raw_data:u32,
    pub pointer_to_relocations:u32,
    pub pointer_to_linenumbers:u32,
    pub num_of_relocations:u16,
    pub num_of_linenumbers:u16,
    pub characteristics:u32,
}

named!(parse_section<&[u8],Section>,do_parse!(
    name: take_str!(8) >>
    virtual_size: le_u32 >>
    virtual_addr: le_u32 >>
    size_of_raw_data: le_u32 >>
    pointer_to_raw_data: le_u32 >>
    pointer_to_relocations: le_u32 >>
    pointer_to_linenumbers: le_u32 >>
    num_of_relocations: le_u16 >>
    num_of_linenumbers: le_u16 >>
    characteristics: le_u32 >>
    (Section{
        name: name,
        virtual_size: virtual_size,
        virtual_addr: virtual_addr,
        size_of_raw_data: size_of_raw_data,
        pointer_to_raw_data: pointer_to_raw_data,
        pointer_to_relocations: pointer_to_relocations,
        pointer_to_linenumbers: pointer_to_linenumbers,
        num_of_relocations: num_of_relocations,
        num_of_linenumbers: num_of_linenumbers,
        characteristics: characteristics
    })
));