use std::rc::Rc;
use std::cell::RefCell;
use crate::rscli::loader::DllFile;
use crate::rscli::meta::CLIData;
use crate::rscli::meta::tbl::*;
use std::fs::OpenOptions;


#[derive(Default)]
pub struct ReflectionInfo {
    dll: Option<Rc<RefCell<DllFile>>>,

    info_assembly: Vec<AssemblyInfo>,
}

impl ReflectionInfo {
    pub fn new() -> ReflectionInfo {
        Default::default()
    }

    pub fn load_dll(&mut self, dll: &Rc<RefCell<DllFile>>) {

        //process dll
        let mut data = &dll.borrow_mut().clidata;
        let filter = |x: &MetaAssembly| x.name.as_ref() == "netdlltest";

        let asm: Option<AssemblyInfo> = data.tbl_assembly.create_runtime_type_by_filter(&filter);
        match asm {
            Some(info) => self.info_assembly.push(info),
            None => (),
        }
        self.dll = Some(dll.clone());
    }
}

pub trait RuntimeInfoType<I, M: MetaItem<M>>
{
    fn new(meta: &M, index: usize) -> I;
}

pub struct AssemblyInfo {
    pub name: Rc<String>,
    pub major_ver: u16,

    meta_index: usize,
}

impl RuntimeInfoType<AssemblyInfo, MetaAssembly> for AssemblyInfo {
    fn new(meta: &MetaAssembly, index: usize) -> AssemblyInfo {
        AssemblyInfo {
            name: Rc::clone(&meta.name),
            major_ver: meta.major_ver,
            meta_index: index,
        }
    }
}