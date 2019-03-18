use std::path::Path;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

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


pub struct DllFile{
    data:Vec<u8>,
}

impl DllFile{
    pub fn new(dat:Vec<u8>)->DllFile{
        let dllfile = DllFile{
            data:dat
        };
        dllfile.parse();
        dllfile
    }

    fn parse(&self){

        let reader = &mut BinaryReader::new(&self.data);
        reader.seek(0);

        WinPe::parse_winpe(reader);

        reader.ate(16);

        let clidata = CLIData::parse_cli_data(reader);

        println!("{:?}",clidata);


    }
}