use crate::rscli::util::reader::BinaryReader;
use crate::rscli::util::reader::DataPointer;

use crate::rscli::meta::tbl::CLIColumnType;
use crate::rscli::meta::tbl::CLITableId;
use crate::rscli::meta::tbl::CLIColumnMap;
use crate::rscli::meta::tbl::*;

use crate::rscli::util::BitUtility;

use std::collections::HashMap;
use std::iter::*;

pub mod tbl;

#[derive(Default,Debug)]
pub struct CLIData{
    pub header:CLIHeader,
    pub meta:CLIMetaData,
    pub tilde_stream: CLITildeStream,

    pub tbl_module:Option<CLITable<MetaModule>>,
}

impl CLIData{

    pub fn default()->CLIData{
        CLIData{
            header: Default::default(),
            meta:Default::default(),
            tilde_stream:Default::default(),
            tbl_module:Option::None,
        }
    }

    pub fn parse_cli_data(reader:&mut BinaryReader)->CLIData{

        let mut clidata:CLIData = Default::default();

        clidata.header = CLIHeader::parse(reader);
        let meta = CLIMetaData::parse(reader);
        clidata.tilde_stream = CLITildeStream::parse(reader);

        let meta_base_addr = meta.meta_pos;

        let (str_off,str_size) = meta.get_stream_rva("#Strings");

        println!("{} {}",meta_base_addr + str_off ,meta_base_addr+str_off + str_size);

        clidata.meta = meta;
        clidata
    }
}

#[derive(Debug,Default)]
pub struct CLIHeader {
    pub major_runtime_ver: u16,
    pub minor_runtime_ver: u16,
    pub metadata: DataPointer,
    pub flags: u32,
    pub entry_point_token: u32,
    pub strong_name_signature: DataPointer,
}

impl CLIHeader{

    pub fn default()->CLIHeader{
        CLIHeader{
            major_runtime_ver:0,
            minor_runtime_ver:0,
            metadata: Default::default(),
            flags:0,
            entry_point_token:0,
            strong_name_signature: Default::default(),
        }
    }

    pub fn parse(reader:&mut BinaryReader)->CLIHeader{
        let mut header=  CLIHeader::default();

        reader.tag(&[0x48,0,0,0]);
        header.major_runtime_ver = reader.le_u16();
        header.minor_runtime_ver = reader.le_u16();
        header.metadata = reader.data_pointer();
        header.flags = reader.le_u32();
        header.entry_point_token = reader.le_u32();
        let resources = reader.data_pointer();
        header.strong_name_signature= reader.data_pointer();
        let code_manager_tbl = reader.ate(8);
        let vtable_fixups = reader.data_pointer();
        let export_addr_tbl_jumps = reader.ate(8);
        let managed_native_header = reader.ate(8);
        header
    }
}

#[derive(Debug,Default)]
pub struct CLIMetaData{
    pub major_version:u16,
    pub minor_version:u16,
    pub cli_ve_str:String,
    pub num_of_stream:u16,
    pub stream_header:Vec<CLIStreamHeader>,
    pub meta_pos:usize,
}

impl CLIMetaData{
    pub fn default()->CLIMetaData{
        CLIMetaData{
            major_version:0,
            minor_version:0,
            cli_ve_str:String::new(),
            num_of_stream:0,
            stream_header:vec![],
            meta_pos:0,
        }
    }

    pub fn parse(reader:& mut BinaryReader)->CLIMetaData{
        let mut metadata:CLIMetaData = Default::default();

        reader.ate_till_tag(&[0x42,0x53,0x4A,0x42]);
        metadata.meta_pos= reader.pos;
        reader.ate(4);

        metadata.major_version = reader.le_u16();
        metadata.minor_version = reader.le_u16();
        reader.ate(4);
        let ver_sz = reader.le_u32();
        metadata.cli_ve_str = reader.str(ver_sz as usize);
        reader.ate(2);
        metadata.num_of_stream = reader.le_u16();

        metadata.stream_header= reader.repeat(CLIStreamHeader::parse,metadata.num_of_stream as u32);

        metadata
    }

    pub fn get_stream_rva(&self,name:&str)->(usize,usize){
        let stream_haeder = &self.stream_header;
        let mut ret:(usize,usize) = (0,0);
        for stream in stream_haeder.iter() {
            if name == stream.name {
                ret = (stream.offset as usize,stream.size as usize);
                break;
            }
        };
        ret
    }
}

#[derive(Debug)]
pub struct CLIStreamHeader{
    pub offset:u32,
    pub size:u32,
    pub name:String,
}

impl CLIStreamHeader{
    pub fn parse(reader:& mut BinaryReader)->CLIStreamHeader{
        let offset = reader.le_u32();
        let size = reader.le_u32();
        let name = reader.str_pad();
        CLIStreamHeader{
            offset:offset,
            size:size,
            name:name
        }
    }
}

#[derive(Debug,Copy, Clone,Default)]
pub struct CLIHeapSize{
    pub string:u8,
    pub guid:u8,
    pub blob:u8,
}

impl CLIHeapSize{
    pub fn new(heapsize:u8)->CLIHeapSize{
        CLIHeapSize{
            string: if heapsize & 0b1 > 0 {4}else{2},
            guid: if heapsize & 0b10 > 0 {4}else{2},
            blob: if heapsize & 0b100 > 0 {4}else{2},
        }
    }
}

#[derive(Debug,Default)]
pub struct CLITildeStream{
    pub major_ver:u8,
    pub minor_ver:u8,
    pub heap_size:CLIHeapSize,
    pub valid:u64,
    pub sorted:u64,
    pub rows: Vec<u32>,


    pub column_size:HashMap<CLIColumnType,u8>,
    pub table_rows:Vec<u32>,
    pub table_valid:Vec<CLITableId>,

}

impl CLITildeStream{

    pub fn parse(reader:&mut BinaryReader)->CLITildeStream{
        let mut tilde:CLITildeStream = Default::default();

        reader.ate(4);
        tilde.major_ver = reader.le_u8();
        tilde.minor_ver = reader.le_u8();

        let raw_heap_size = reader.le_u8();
        tilde.heap_size = CLIHeapSize::new(raw_heap_size);

        reader.tag(&[0x01]);
        tilde.valid= reader.le_u64();
        tilde.sorted = reader.le_u64();

        let table_count = BitUtility::bits_count_u64(tilde.valid) as u32;
        tilde.rows = reader.repeat(BinaryReader::le_u32,table_count);
        tilde.calculate_table_data();

        tilde
    }

    fn calculate_table_data(&mut self){
        let table_count= self.rows.len();
        let mut table_rows:Vec<u32> = vec![0;64];
        let rows = &self.rows;
        let mut table_map = CLITableId::map();
        table_map.sort();
        let valid = self.valid;
        let mut index:usize = 0;
        for (t,&tableid) in table_map.iter().enumerate(){
            if valid & (1 << tableid as u8) > 0 {
                self.table_valid.push(tableid);
                table_rows[t] = rows[index];
                index +=1;
            }
        }

        //column rows
        let mut column_size:HashMap<CLIColumnType,u8> = HashMap::new();
        let column_map = &CLIColumnMap;
        for (&column,table_vec) in column_map.iter() {

            let bit_count = (table_vec.len() as f32).log2().ceil() as u8;
            let mut tbl_max_row = 0_u32;
            for &tblid in table_vec.iter(){
                if tblid != CLITableId::Invalid {
                    tbl_max_row = tbl_max_row.max(table_rows[tblid as usize])
                }
            }

            let byte_size:u8 = if tbl_max_row > (1 << (16 - bit_count)) {
                4
            }else{
                2
            };
            column_size.insert(column,byte_size);
        }

        self.table_rows = table_rows;
        self.column_size = column_size;
    }

    fn get_table_row(self:&Self,table_id:CLITableId)->u32{
        self.table_rows[table_id as usize]
    }

    fn get_column_byte(self:&Self,column:CLIColumnType)->u8{
        self.column_size[&column]
    }

//    fn parse_tables(&mut self,reader:&mut BinaryReader){
//
//        self.tbl_module = Some(MetaModule::parse_table(reader,self));
//
//    }

}


pub struct CLIStringStream<'a>{
    pub data:Vec<&'a str>,
}

impl<'a> CLIStringStream<'a>{
    pub fn parse(reader:&'a mut BinaryReader,stream_info:(usize,usize))->CLIStringStream<'a>{
        let cur_pos= reader.pos;
        reader.seek(stream_info.0+1);

        let max_addr = stream_info.1;

        let mut str_vec:Vec<&str> = Vec::new();

//        loop {
//            let str = reader.str_read_ref();
//            if str.is_none(){
//                break;
//            }else{
//                str_vec.push(str.unwrap());
//            }
//        };

        CLIStringStream{
            data:str_vec
        }
    }
}
