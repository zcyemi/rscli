
use crate::rscli::util::reader::BinaryReader;

#[derive(Debug)]
pub struct WinPe{
    pub num_section:u16,
    pub time_date_stamp:u32,
    pub pointer_sbl_tbl:u32,
    pub sz_opt_header:u16,
    pub characteristics:u16,

    pub maj_linker_ver:u8,
    pub min_linker_ver:u8,
    pub size_code:u32,
    pub size_initialized_data:u32,
    pub size_uninitialized_data:u32,
    pub addr_entry_point:u32,
    pub base_of_code:u32,
    pub base_of_data:u32,

}

impl WinPe{

    pub fn parse_winpe(reader:&mut BinaryReader)->WinPe{

        //dos header
        reader.tag(&[0x4D,0x5A]);
        reader.ate(62);

        //dos stub
        reader.ate(64);

        //COFF header
        reader.tag(&[0x50,0x45,0,0]);
        let machine = reader.le_u16();
        let num_section = reader.le_u16();
        let time_date_stamp = reader.le_u32();
        let pointer_sbl_tbl = reader.le_u32();
        let num_sbl_tbl = reader.le_u32();
        let sz_opt_header =reader.le_u16();
        let characteristics = reader.le_u16();

        //COFF field
        reader.tag(&[0x0B,0x01]);
        let maj_linker_ver = reader.le_u8();
        let min_linker_ver = reader.le_u8();
        let size_code = reader.le_u32();
        let size_initialized_data = reader.le_u32();
        let size_uninitialized_data = reader.le_u32();
        let addr_entry_point = reader.le_u32();
        let base_of_code = reader.le_u32();
        let base_of_data = reader.le_u32();
        //pe nt field
        reader.ate(68);

        //data directories
        let export_tbl = reader.data_pointer();
        let import_tbl = reader.data_pointer();
        let resource_tbl = reader.data_pointer();
        let exception_tbl = reader.data_pointer();
        let certificate_tbl = reader.data_pointer();
        let base_relocation_tbl =reader.data_pointer();
        let debug = reader.data_pointer();
        let architecture_data = reader.data_pointer();
        let global_ptr = reader.le_u32();
        reader.ate(4);

        let tls_tbl = reader.data_pointer();
        let load_config_tbl = reader.data_pointer();
        let bound_import = reader.data_pointer();
        let import_addr_tbl = reader.data_pointer();
        let delay_import_descriptor = reader.data_pointer();
        let clr_runtime_helper = reader.data_pointer();
        reader.ate(8);

        //sections
        let text_section = WinPe::parse_section(reader);
        let rsrc_section = WinPe::parse_section(reader);
        let reloc_section = WinPe::parse_section(reader);

        WinPe{
            num_section,
            time_date_stamp,
            pointer_sbl_tbl,
            sz_opt_header,
            characteristics,
            maj_linker_ver,
            min_linker_ver,
            size_code,
            size_initialized_data,
            size_uninitialized_data,
            addr_entry_point,
            base_of_code,
            base_of_data
        }
    }

    fn parse_section(reader:&mut BinaryReader){
        let virtual_size = reader.le_u32();
        let virtual_addr = reader.le_u32();
        println!("section addr: {:?}",virtual_addr);
        let size_of_raw_data = reader.le_u32();
        let pointer_to_raw_data = reader.le_u32();
        let pointer_to_relocations = reader.le_u32();
        let pointer_to_linenumbers = reader.le_u32();
        let num_of_relocations = reader.le_u16();
        let num_of_linenumbers = reader.le_u16();
        let characteristics = reader.le_u32();
    }
}

