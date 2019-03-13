pub mod winpe;
pub mod cli;
pub mod util;

use winpe::{win_pe,WinPE};
use cli::{parse_clidata,CLIData};

use nom::{IResult,HexDisplay,be_u8,le_u8,le_u16,le_u32};

named!(parser_dll<&[u8],DllFile>,do_parse!(
    pe: win_pe >>
    take!(16) >>
    clidata: parse_clidata >>
    (DllFile{
        pe:pe,
        cli:clidata,
    })
));

#[derive(Debug)]
pub struct DllFile<'a>{
    pub pe: WinPE<'a>,
    pub cli: CLIData<'a>,
}

pub fn loader_test(){

    let data = include_bytes!("E:/netdlltest.dll");
    let d = parser_dll(data);

    match d {
        Ok((i,dll))=>{
            println!("ok");
            println!("{:?}",dll);
        },
        _ =>{
            println!("fail")
        }
    }

}

