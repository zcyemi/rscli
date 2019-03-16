pub mod winpe;
pub mod cli;
pub mod util;

use winpe::WinPE;
use cli::CLIData;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug)]
pub struct DllFile<'a>{
    pub pe: WinPE<'a>,
    pub cli: CLIData<'a>,
}

impl<'a> DllFile<'a>{

    pub fn load(input:&[u8])->Option<DllFile>{
        let ret= match DllFile::parse(input) {
            Ok(val)=>{
                Some(val.1)
            },
            Err(e)=>{
                println!("{}",e);
                Option::None
            }
        };
        ret
    }
    named!(parse<&[u8],DllFile>,do_parse!(
        pe: call!(WinPE::parse) >>
        take!(16) >>
        clidata: call!(CLIData::parse) >>
        (DllFile{
            pe:pe,
            cli:clidata,
        })
    ));
}


pub fn loader_test(){

//    let data = include_bytes!("D:/TestDll.dll");
//
//    let dll = DllFile::load(data).unwrap();
//
//    print!("{:?}",dll);

    let path = Path::new("D:/TestDll.dll");
    let display = path.display();

    let mut file = match File::open(&path){
        Err(why)=> panic!("count not open file: {}",why.description()),
        Ok(file)=> file
    };

    let mut data:Vec<u8> = Vec::new();


    let result = match file.read_to_end(&mut data) {
        Err(why)=> panic!("count not read file: {}",why.description()),
        Ok(_)=> println!("{} contains: \n {}",display,data.len()),
    };

}

