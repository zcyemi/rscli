use nom::{IResult, le_u8, le_u16, le_u32, le_u64};

use crate::rscil::loader::util::DataInfo;
use crate::rscil::loader::util::parse_str_pad;
use crate::rscil::loader::util::calculate_bits_u64;
use crate::rscil::loader::util::resolve_result;
use crate::rscil::loader::util::return_err;
use crate::rscil::loader::util::parse_id;

use std::cmp::Ordering;
use std::collections::HashMap;
use std::*;
use std::hash::{Hash,Hasher};

#[derive(Debug)]
pub struct CLIData<'a> {
    pub header: CLIHeader,
    pub metadata: CLIMetadata<'a>,

}

impl<'a> CLIData<'a> {
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
pub struct CLIHeader {
    pub major_runtime_ver: u16,
    pub minor_runtime_ver: u16,
    pub metadata: DataInfo,
    pub flags: u32,
    pub entry_point_token: u32,
    pub strong_name_signature: DataInfo,
}

impl CLIHeader {
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
pub struct CLIMetadata<'a> {
    pub major_version: u16,
    pub minor_version: u16,
    pub cli_ver_str: &'a str,
    pub num_of_stream: u16,
    pub stream_header: Vec<CLIStreamHeader<'a>>,
    pub tilde_stream: CLITildeStream,
}

impl<'a> CLIMetadata<'a> {
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
        tilde_stream: call!(CLITildeStream::parse) >>
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
pub struct CLIStreamHeader<'a> {
    pub offset: u32,
    pub size: u32,
    pub name: &'a str,
}

impl<'a> CLIStreamHeader<'a> {
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
pub struct CLITildeStream {
    pub major_ver: u8,
    pub minor_ver: u8,
    pub heap_size: u8,
    pub valid: u64,
    pub sorted: u64,
    pub rows: Vec<u32>,

    pub table_module: Option<Box<ModuleTbl>>,
}


impl CLITildeStream {
    pub fn default() -> CLITildeStream {
        CLITildeStream {
            major_ver: 0,
            minor_ver: 0,
            heap_size: 0,
            valid: 0,
            sorted: 0,
            rows: vec![],
            table_module: Option::None,
        }
    }

    pub fn parse(input: &[u8]) -> IResult<&[u8], CLITildeStream> {
        let mut tildestream = CLITildeStream::default();

        type DataPartial = (u8, u8, u8, u64, u64);
        let ret: IResult<&[u8], DataPartial> = do_parse!(input,
            take!(4) >>
            major_ver: le_u8 >>
            minor_ver: le_u8 >>
            heap_size: le_u8 >>
            tag!(&[0x01]) >>
            valid: le_u64>>
            sorted: le_u64 >>
            (major_ver,minor_ver,heap_size,valid,sorted));

        if ret.is_err() {
            return_err(ret.unwrap().0, 0)
        } else {
            let (i, o) = ret.unwrap();

            tildestream.major_ver = o.0;
            tildestream.minor_ver = o.1;
            tildestream.heap_size = o.2;
            tildestream.valid = o.3;
            tildestream.sorted = o.4;

            let (major_ver, minor_ver, heap_size, valid, sorted) = o;
            let tblcount = calculate_bits_u64(valid) as usize;

            println!("table count:{}", tblcount);

            let mut suc = false;

            let ret = resolve_result(&mut suc, count!(i,le_u32,tblcount));
            if !suc {
                return_err(ret.unwrap().0, 10)
            } else {
                let rows = ret.unwrap();

                tildestream.rows = rows.1;
                let input = tildestream.parse_table(rows.0);
                Result::Ok((input, tildestream))
            }
        }
    }

    fn parse_table<'a>(self: &mut Self, input: &'a [u8]) -> &'a [u8] {
        let mut input = input;

        let valid = &self.valid;
        println!("{:#b}", *valid);

        let mut tbls = TableId::map();
        tbls.sort();

        let tbl_iter = tbls.iter();
        let heapsize =CLIHeapSize::new( self.heap_size);


        let mut suc = false;

        let mut tables:Vec<TableId> = Vec::new();

        for (ind, &iter) in tbl_iter.enumerate() {
            if TableId::is_table_valid(&self.valid, &iter) {
                println!("has table {:?}", iter);
                tables.push(iter);
            }
        }

        let column_byte_size = parse_cli_column_size(tables,&self.rows);

        println!("column_byte_size: {:?}",column_byte_size);

        let tblModule = resolve_result(&mut suc, ModuleTbl::parse(input,heapsize, self.rows[0]));
        if suc {
            let tbl = tblModule.unwrap();
            input = tbl.0;
            self.table_module = Some(Box::new(tbl.1));
        }
        input
    }
}

#[derive(Default,Copy, Clone)]
pub struct CLIHeapSize{
    pub string:bool,
    pub guid:bool,
    pub blob:bool,
}

impl CLIHeapSize{
    pub fn new(heapsize:u8)->CLIHeapSize{
        CLIHeapSize{
            string:heapsize & 0b1 > 0,
            guid: heapsize & 0b10 > 0,
            blob: heapsize & 0b100 > 0,
        }
    }
    fn default() -> Self {
        CLIHeapSize{string:false,guid:false,blob:false}
    }
}

#[derive(Eq,Debug,PartialEq,Hash)]
pub enum CLIColumnType{
    TypeDefOrRef =0,
    HasConstant = 1,
    HasCustomAttribute = 2,
    HasFieldMarshall = 3,
    HasDeclSecurity = 4,
    MemberRefParent = 5,
    HasSemantics = 6,
    MethodDefOrRef =7,
    MemberForwarded = 8,
    Implementation = 9,
    CustomAttributeType = 10,
    ResolutionScope = 11,
    TypeOrMethodDef = 12,
}




pub fn parse_cli_column_size(tblVec:Vec<TableId>,tblrows:&Vec<u32>)-> HashMap<CLIColumnType,u8>{
    let mut hashmap:HashMap<CLIColumnType,u8> = HashMap::new();

    let mut rowvec:[u32;64] = [0;64];

    for (i,tbl) in tblVec.iter().enumerate(){
        rowvec[*tbl as usize] = tblrows[i];
    }

    let rowvec = rowvec.to_vec();


    hashmap.insert(CLIColumnType::TypeDefOrRef,parse_cli_column_item(&rowvec,vec![
        TableId::TypeDef,
        TableId::TypeRef,
        TableId::TypeSpec,
    ]));
    hashmap.insert(CLIColumnType::HasConstant,parse_cli_column_item(&rowvec,vec![
        TableId::Field,
        TableId::Param,
        TableId::Property,
    ]));
    hashmap.insert(CLIColumnType::HasCustomAttribute,parse_cli_column_item_extra(&rowvec,vec![
        TableId::MethodDef,
        TableId::Field,
        TableId::TypeRef,
        TableId::TypeDef,
        TableId::Param,
        TableId::InterfaceImpl,
        TableId::MemberRef,
        TableId::Module,
        //TableId::Permission,
        TableId::Property,
        TableId::Event,
        TableId::StandAloneSig,
        TableId::ModuleRef,
        TableId::TypeSpec,
        TableId::Assembly,
        TableId::AssemblyRef,
        TableId::File,
        TableId::ExportedType,
        TableId::ManifestResource,
        TableId::GenericParam,
        TableId::GenericParamConstraint,
        TableId::MethodSpec,
    ],1));
    hashmap.insert(CLIColumnType::HasFieldMarshall,parse_cli_column_item(&rowvec,vec![
        TableId::Field,
        TableId::Param,
    ]));
    hashmap.insert(CLIColumnType::HasDeclSecurity,parse_cli_column_item(&rowvec,vec![
        TableId::TypeDef,
        TableId::MethodDef,
        TableId::Assembly,
    ]));
    hashmap.insert(CLIColumnType::MemberRefParent,parse_cli_column_item(&rowvec,vec![
        TableId::TypeDef,
        TableId::TypeRef,
        TableId::ModuleRef,
        TableId::MethodDef,
        TableId::TypeSpec
    ]));

    hashmap.insert(CLIColumnType::HasSemantics,parse_cli_column_item(&rowvec,vec![
        TableId::Event,
        TableId::Property,
    ]));
    hashmap.insert(CLIColumnType::MethodDefOrRef,parse_cli_column_item(&rowvec,vec![
        TableId::MethodDef,
        TableId::MemberRef,
    ]));
    hashmap.insert(CLIColumnType::MemberForwarded,parse_cli_column_item(&rowvec,vec![
        TableId::Field,
        TableId::MethodDef,
    ]));
    hashmap.insert(CLIColumnType::Implementation,parse_cli_column_item(&rowvec,vec![
        TableId::Field,
        TableId::AssemblyRef,
        TableId::ExportedType,
    ]));
    hashmap.insert(CLIColumnType::CustomAttributeType,parse_cli_column_item_extra(&rowvec,vec![
        TableId::MethodDef,
        TableId::MemberRef,
    ],3_u8));
    hashmap.insert(CLIColumnType::ResolutionScope,parse_cli_column_item(&rowvec,vec![
        TableId::Module,
        TableId::ModuleRef,
        TableId::AssemblyRef,
        TableId::TypeRef,
    ]));
    hashmap.insert(CLIColumnType::TypeOrMethodDef,parse_cli_column_item(&rowvec,vec![
        TableId::TypeDef,
        TableId::MethodDef,
    ]));
    hashmap

}


pub fn parse_cli_column_item(tblRows:&Vec<u32>,columnTbl:Vec<TableId>)->u8{
    parse_cli_column_item_extra(tblRows,columnTbl,0)
}

pub fn parse_cli_column_item_extra(tblRows:&Vec<u32>,columnTbl:Vec<TableId>,extra_tag_count:u8)->u8
{
    let bittag = ((columnTbl.len() + extra_tag_count as usize) as f32).log2().ceil() as u8;

    let mut maxrow =0;
    for &col in columnTbl.iter() {
        let tbl_row = tblRows[col as usize];
        maxrow = tbl_row.max(maxrow);
    }

    let mut bytesize:u8 = 2;
    if maxrow > (1 <<(16-bittag)){
        bytesize = 4;
    }
    bytesize
}


#[derive(Debug, Copy, Clone, Eq)]
pub enum TableId {
    Assembly = 0x20,
    AssemblyOS = 0x22,
    AssemblyProcessor = 0x21,
    AssemblyRef = 0x23,
    AssemblyRefOS = 0x25,
    AssemblyRefProcessor = 0x24,
    ClassLayout = 0x0F,
    Constant = 0x0B,
    CustomAttribute = 0x0C,
    DeclSecurity = 0x0E,
    EventMap = 0x12,
    Event = 0x14,
    ExportedType = 0x27,
    Field = 0x04,
    FieldLayout = 0x10,
    FieldMarshal = 0x0D,
    FieldRVA = 0x1D,
    File = 0x26,
    GenericParam = 0x2A,
    GenericParamConstraint = 0x2C,
    ImplMap = 0x1C,
    InterfaceImpl = 0x09,
    ManifestResource = 0x28,
    MemberRef = 0x0A,
    MethodDef = 0x06,
    MethodImpl = 0x19,
    MethodSemantics = 0x18,
    MethodSpec = 0x2B,
    Module = 0x00,
    ModuleRef = 0x1A,
    NestedClass = 0x29,
    Param = 0x08,
    Property = 0x17,
    PropertyMap = 0x15,
    StandAloneSig = 0x11,
    TypeDef = 0x02,
    TypeRef = 0x01,
    TypeSpec = 0x1B,
}

impl Ord for TableId {
    fn cmp(&self, other: &TableId) -> Ordering {
        (*self as u8).cmp(&(*other as u8))
    }
}

impl PartialOrd for TableId {
    fn partial_cmp(&self, other: &TableId) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for TableId {
    fn eq(&self, other: &TableId) -> bool {
        (*self as u8) == (*other as u8)
    }
}

impl TableId {
    pub fn is_table_valid(valid: &u64, id: &TableId) -> bool {
        valid & (1 << (*id as u64)) > 0
    }

    pub fn map() -> [TableId; 36] {
        static TABLES: [TableId; 36] = [
            TableId::EventMap,
            TableId::Assembly,
            TableId::AssemblyOS,
            TableId::AssemblyProcessor,
            TableId::AssemblyRef,
            TableId::AssemblyRefOS,
            TableId::AssemblyRefProcessor,
            TableId::ClassLayout,
            TableId::Constant,
            TableId::CustomAttribute,
            TableId::DeclSecurity,
            TableId::EventMap,
            TableId::ExportedType,
            TableId::Field,
            TableId::FieldLayout,
            TableId::FieldMarshal,
            TableId::File,
            TableId::GenericParam,
            TableId::GenericParamConstraint,
            TableId::ImplMap,
            TableId::ManifestResource,
            TableId::MemberRef,
            TableId::MethodDef,
            TableId::MethodImpl,
            TableId::MethodSemantics,
            TableId::MethodSpec,
            TableId::Module,
            TableId::ModuleRef,
            TableId::NestedClass,
            TableId::Param,
            TableId::Property,
            TableId::PropertyMap,
            TableId::StandAloneSig,
            TableId::TypeDef,
            TableId::TypeRef,
            TableId::TypeSpec,
        ];
        TABLES
    }
}


type Index = u32;

#[derive(Debug)]
pub struct ModuleTbl {
    pub row: Index,
    pub data: Vec<MetaModule>,
}

impl ModuleTbl {
    named_args!(pub parse(hs:CLIHeapSize,rows:u32)<&[u8],ModuleTbl>,do_parse!(
        meta: count!(call!(MetaModule::parse,hs),rows as usize) >>
        (ModuleTbl{
            row: rows,
            data: meta
        })
    ));
}

#[derive(Debug)]
pub struct MetaModule {
    pub name: Index,
    pub mvid: Index,
}

impl MetaModule {
    named_args!(pub parse(hs:CLIHeapSize)<&[u8],MetaModule>,do_parse!(
       take!(2) >>
       name: call!(parse_id,hs.string) >>
       mvid: call!(parse_id,hs.guid) >>
       call!(parse_id,hs.guid) >>
       call!(parse_id,hs.guid) >>
       (MetaModule{
            name:name,
            mvid:mvid,
       })
    ));
}


pub trait CLIDataItem<T>{
    fn parse<'a>(i:&'a [u8],heapsize:&'a CLIHeapSize)->IResult<&'a [u8],T>;
}

pub struct CLITbl<D> where D: CLIDataItem<D>
{
    pub row:u32,
    pub data: Vec<D>,
}

impl<D> CLITbl<D> where D: CLIDataItem<D>
{
    named_args!(pub parse<'a>(hs:&'a CLIHeapSize,rows:u32)<&'a [u8],CLITbl<D>>,do_parse!(
        meta: count!(call!(D::parse,hs),rows as usize) >>
        (CLITbl::<D>{
            row: rows,
            data: meta
        })
    ));
}

pub struct TypeRef{
}

impl CLIDataItem<TypeRef> for TypeRef{
    fn parse<'a>(i: &'a [u8], heapsize:&'a CLIHeapSize) -> IResult<&'a [u8],TypeRef> {
        unimplemented!()
    }
}

//
//pub fn xx(input:&[u8]){
//
//    let heapsize:CLIHeapSize = Default::default();
//    let x = CLITbl::<TypeRef>::parse(input,&heapsize,10);
//}
//
