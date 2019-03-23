use std::rc::Rc;
use std::cell::RefCell;
use crate::rscli::loader::DllFile;
use crate::rscli::meta::CLIData;
use crate::rscli::meta::tbl::*;
use std::fs::OpenOptions;


#[derive(Default)]
pub struct ReflectionInfo{
    dll:Option<Rc<RefCell<DllFile>>>,
}

impl ReflectionInfo{
    pub fn new()->ReflectionInfo{
        ReflectionInfo{dll:None}
    }

    pub fn load_dll(&mut self,dll:&Rc<RefCell<DllFile>>){
        self.dll= Some(dll.clone());
    }
}

pub struct AssemblyInfo{
    pub name:Rc<String>,
    pub major_ver:u16,

    meta_index:usize,
}

impl AssemblyInfo{

    pub fn new(meta:&MetaAssembly,index:usize)->AssemblyInfo{
        AssemblyInfo{
            name:Rc::clone(&meta.name),
            major_ver: meta.major_ver,
            meta_index:index
        }
    }
}