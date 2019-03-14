pub mod winpe;
pub mod cli;
pub mod util;

use winpe::WinPE;
use cli::CLIData;

use nom::{IResult,HexDisplay,be_u8,le_u8,le_u16,le_u32};

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

    let data = include_bytes!("D:/TestDll.dll");

    let dll = DllFile::load(data).unwrap();

    print!("{:?}",dll);

}

