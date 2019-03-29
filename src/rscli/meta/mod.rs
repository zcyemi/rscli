use crate::rscli::util::reader::BinaryReader;
use crate::rscli::util::reader::DataPointer;

use crate::rscli::meta::tbl::CLIColumnType;
use crate::rscli::meta::tbl::CLITableId;
use crate::rscli::meta::tbl::CLICOLUMN_MAP;
use crate::rscli::meta::tbl::*;

use crate::rscli::util::BitUtility;

use std::collections::HashMap;
use std::rc::Rc;
use crate::rscli::winpe::WinPe;

pub mod tbl;


#[derive(Default, Debug)]
pub struct CLIData {
    pub header: CLIHeader,
    pub meta: CLIMetaData,
    pub tilde_stream: CLITildeStream,

    pub addr_offset_code: usize,

    pub string_stream: CLIStringStream,

    pub tbl_module: CLITable<MetaModule>,
    pub tbl_typeref: CLITable<MetaTypeRef>,
    pub tbl_typedef: CLITable<MetaTypeDef>,
    pub tbl_methoddef: CLITable<MetaMethodDef>,
    pub tbl_member_ref: CLITable<MetaMemberRef>,
    pub tbl_custom_attribute: CLITable<MetaCustomAttribute>,
    pub tbl_stand_alone_sig: CLITable<MetaStandAloneSig>,
    pub tbl_assembly: CLITable<MetaAssembly>,
    pub tbl_assembly_ref: CLITable<MetaAssemblyRef>,

}

impl CLIData {
    pub fn parse_cli_data(reader: &mut BinaryReader, pe: &WinPe) -> CLIData {
        let mut clidata: CLIData = Default::default();

        clidata.header = CLIHeader::parse(reader);
        let meta = CLIMetaData::parse(reader);
        clidata.tilde_stream = CLITildeStream::parse(reader);

        let meta_base_addr = meta.meta_pos;

        let (str_off, str_size) = meta.get_stream_rva("#Strings");
        let str_start = meta_base_addr + str_off;
        let str_end = str_start + str_size;
        let string_stream = CLIStringStream::parse(reader, (str_start, str_end));

        clidata.string_stream = string_stream;
        clidata.addr_offset_code = (pe.base_of_code - 0x200) as usize;

        clidata.parse_tables(reader);


        clidata.meta = meta;
        clidata
    }

    fn parse_tables(&mut self, reader: &mut BinaryReader) {
        let tilde_stream = &self.tilde_stream;
        let string_stream = &self.string_stream;
        self.tbl_module = MetaModule::parse_table(reader, tilde_stream, string_stream);
        self.tbl_typeref = MetaTypeRef::parse_table(reader, tilde_stream, string_stream);
        self.tbl_typedef = MetaTypeDef::parse_table(reader, tilde_stream, string_stream);
        self.tbl_methoddef = MetaMethodDef::parse_table(reader, tilde_stream, string_stream);
        self.tbl_member_ref = MetaMemberRef::parse_table(reader, tilde_stream, string_stream);
        self.tbl_custom_attribute = MetaCustomAttribute::parse_table(reader, tilde_stream, string_stream);
        self.tbl_stand_alone_sig = MetaStandAloneSig::parse_table(reader, tilde_stream, string_stream);
        self.tbl_assembly = MetaAssembly::parse_table(reader, tilde_stream, string_stream);
        self.tbl_assembly_ref = MetaAssemblyRef::parse_table(reader, tilde_stream, string_stream);
//        println!("module end{:#x}",reader.pos);
    }

    pub fn get_rva_addr(&self, rva: usize) -> usize {
        rva - self.addr_offset_code
    }
}

#[derive(Debug, Default)]
pub struct CLIHeader {
    pub major_runtime_ver: u16,
    pub minor_runtime_ver: u16,
    pub metadata: DataPointer,
    pub flags: u32,
    pub entry_point_token: u32,
    pub strong_name_signature: DataPointer,
}

impl CLIHeader {
    pub fn default() -> CLIHeader {
        CLIHeader {
            major_runtime_ver: 0,
            minor_runtime_ver: 0,
            metadata: Default::default(),
            flags: 0,
            entry_point_token: 0,
            strong_name_signature: Default::default(),
        }
    }

    pub fn parse(reader: &mut BinaryReader) -> CLIHeader {
        let mut header = CLIHeader::default();

        reader.tag(&[0x48, 0, 0, 0]);
        header.major_runtime_ver = reader.le_u16();
        header.minor_runtime_ver = reader.le_u16();
        header.metadata = reader.data_pointer();
        header.flags = reader.le_u32();
        header.entry_point_token = reader.le_u32();
        let _resources = reader.data_pointer();
        header.strong_name_signature = reader.data_pointer();
        let _code_manager_tbl = reader.ate(8);
        let _vtable_fixups = reader.data_pointer();
        let _export_addr_tbl_jumps = reader.ate(8);
        let _managed_native_header = reader.ate(8);
        header
    }
}

#[derive(Debug, Default)]
pub struct CLIMetaData {
    pub major_version: u16,
    pub minor_version: u16,
    pub cli_ve_str: String,
    pub num_of_stream: u16,
    pub stream_header: Vec<CLIStreamHeader>,
    pub meta_pos: usize,
}

impl CLIMetaData {
    pub fn default() -> CLIMetaData {
        CLIMetaData {
            major_version: 0,
            minor_version: 0,
            cli_ve_str: String::new(),
            num_of_stream: 0,
            stream_header: vec![],
            meta_pos: 0,
        }
    }

    pub fn parse(reader: &mut BinaryReader) -> CLIMetaData {
        let mut metadata: CLIMetaData = Default::default();

        reader.ate_till_tag(&[0x42, 0x53, 0x4A, 0x42]);
        metadata.meta_pos = reader.pos;
        reader.ate(4);

        metadata.major_version = reader.le_u16();
        metadata.minor_version = reader.le_u16();
        reader.ate(4);
        let ver_sz = reader.le_u32();
        metadata.cli_ve_str = reader.str(ver_sz as usize);
        reader.ate(2);
        metadata.num_of_stream = reader.le_u16();

        metadata.stream_header = reader.repeat(CLIStreamHeader::parse, metadata.num_of_stream as u32);
        metadata
    }

    pub fn get_stream_rva(&self, name: &str) -> (usize, usize) {
        let stream_haeder = &self.stream_header;
        let mut ret: (usize, usize) = (0, 0);
        for stream in stream_haeder.iter() {
            if name == stream.name {
                ret = (stream.offset as usize, stream.size as usize);
                break;
            }
        };
        ret
    }
}

#[derive(Debug)]
pub struct CLIStreamHeader {
    pub offset: u32,
    pub size: u32,
    pub name: String,
}

impl CLIStreamHeader {
    pub fn parse(reader: &mut BinaryReader) -> CLIStreamHeader {
        let offset = reader.le_u32();
        let size = reader.le_u32();
        let name = reader.str_pad();
        CLIStreamHeader {
            offset: offset,
            size: size,
            name: name,
        }
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct CLIHeapSize {
    pub string: u8,
    pub guid: u8,
    pub blob: u8,
}

impl CLIHeapSize {
    pub fn new(heapsize: u8) -> CLIHeapSize {
        CLIHeapSize {
            string: if heapsize & 0b1 > 0 { 4 } else { 2 },
            guid: if heapsize & 0b10 > 0 { 4 } else { 2 },
            blob: if heapsize & 0b100 > 0 { 4 } else { 2 },
        }
    }
}

#[derive(Debug, Default)]
pub struct CLITildeStream {
    pub major_ver: u8,
    pub minor_ver: u8,
    pub heap_size: CLIHeapSize,
    pub valid: u64,
    pub sorted: u64,
    pub rows: Vec<u32>,

    pub column_size: HashMap<CLIColumnType, u8>,
    pub table_rows: Vec<u32>,
    pub table_valid: Vec<CLITableId>,

}

impl CLITildeStream {
    pub fn parse(reader: &mut BinaryReader) -> CLITildeStream {
        let mut tilde: CLITildeStream = Default::default();

        reader.ate(4);
        tilde.major_ver = reader.le_u8();
        tilde.minor_ver = reader.le_u8();

        let raw_heap_size = reader.le_u8();
        tilde.heap_size = CLIHeapSize::new(raw_heap_size);

        reader.tag(&[0x01]);
        tilde.valid = reader.le_u64();
        tilde.sorted = reader.le_u64();

        let table_count = BitUtility::bits_count_u64(tilde.valid) as u32;
        tilde.rows = reader.repeat(BinaryReader::le_u32, table_count);
        tilde.calculate_table_data();

        tilde
    }

    fn calculate_table_data(&mut self) {
        let _table_count = self.rows.len();
        let mut table_rows: Vec<u32> = vec![0; 64];
        let rows = &self.rows;
        let mut table_map = CLITableId::map();
        table_map.sort();
        let valid = self.valid;
        let mut index: usize = 0;
        for &tableid in table_map.iter() {
            if valid & (1 << tableid as u8) > 0 {
                self.table_valid.push(tableid);
                table_rows[tableid as usize] = rows[index];
                index += 1;
            }
        }

        //column rows
        let mut column_size: HashMap<CLIColumnType, u8> = HashMap::new();
        let column_map = &CLICOLUMN_MAP;
        for (&column, table_vec) in column_map.iter() {
            let bit_count = (table_vec.len() as f32).log2().ceil() as u8;
            let mut tbl_max_row = 0_u32;
            for &tblid in table_vec.iter() {
                if tblid != CLITableId::Invalid {
                    tbl_max_row = tbl_max_row.max(table_rows[tblid as usize])
                }
            }

            let byte_size: u8 = if tbl_max_row > (1 << (16 - bit_count)) {
                4
            } else {
                2
            };
            column_size.insert(column, byte_size);
        }

        self.table_rows = table_rows;
        self.column_size = column_size;
    }

    fn get_table_row(self: &Self, table_id: CLITableId) -> u32 {
        self.table_rows[table_id as usize]
    }

    fn get_column_byte(self: &Self, column: CLIColumnType) -> u8 {
        self.column_size[&column]
    }
}

#[derive(Debug, Default)]
pub struct CLIStringStream {
    pub data: Vec<Rc<String>>,
    pub index_map: HashMap<u32, u32>,
}

impl CLIStringStream {
    pub fn parse(reader: &mut BinaryReader, stream_info: (usize, usize)) -> CLIStringStream {
        let max_addr = stream_info.1;
        let start_addr = stream_info.0;

        let prev_pos = reader.pos;
        reader.seek(start_addr + 1);

        let mut data: Vec<Rc<String>> = Vec::new();
        let mut index_map = HashMap::new();

        let str_empty = Rc::new(String::from(""));
        data.push(str_empty);
        index_map.insert(0, 0);

        let mut str_count = 1;
        let mut str_pos: u32 = 1;
        while reader.pos < max_addr {
            let str = reader.str_read();
            if str.is_none() {
                break;
            } else {
                index_map.insert(str_pos, str_count);
                str_pos = (reader.pos - start_addr) as u32;
                data.push(Rc::new(str.unwrap()));
                str_count += 1;
            }
        }
        reader.seek(prev_pos);

        CLIStringStream {
            data,
            index_map,
        }
    }

    pub fn get_str_by_index(&self, ind: u32) -> Rc<String> {
        let index = self.index_map.get(&ind);
        let val = if index.is_none() {
            Rc::clone(&self.data[0])
        } else {
            let index = index.unwrap();
            Rc::clone(&self.data[*index as usize])
        };
        val
    }
}
