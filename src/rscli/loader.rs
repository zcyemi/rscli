use std::path::Path;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;

use crate::rscli::util::reader::BinaryReader;
use crate::rscli::winpe::WinPe;
use crate::rscli::meta::CLIData;

pub fn load_dll(file_path:&str)->DllFile{
    let path = Path::new(file_path);
    let mut file = match File::open(&path) {
        Err(e)=> panic!("can not open file: {}", e.description()),
        Ok(file)=> file,
    };

    let mut data = vec![];
    let result = match file.read_to_end(&mut data) {
        Err(_e)=> panic!("read file failed: {}",_e.description()),
        Ok(size)=>{size},
    };
    DllFile::new(data)
}

#[derive(Debug,Default)]
pub struct DllFile{
    pub data:Vec<u8>,
    pub clidata:Box<CLIData>,
}

impl DllFile{
    pub fn new(dat:Vec<u8>)->DllFile{

        let reader = &mut BinaryReader::new(&dat);
        reader.seek(0);

        let pe = WinPe::parse_winpe(reader);
        reader.ate(16);

        let cli = Box::new(CLIData::parse_cli_data(reader,&pe));
        DllFile{
            data:dat,
            clidata:cli
        }
    }
}