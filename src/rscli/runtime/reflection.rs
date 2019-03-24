use std::rc::Rc;
use std::cell::RefCell;
use crate::rscli::loader::DllFile;
use crate::rscli::meta::CLIData;
use crate::rscli::meta::tbl::*;
use std::fs::OpenOptions;
use crate::rscli::meta::tbl::CLITableId::TypeDef;


#[derive(Default)]
pub struct ReflectionInfo {
    dll: Option<Rc<RefCell<DllFile>>>,

    info_assembly: Vec<AssemblyInfo>,
    info_types:Vec<TypeInfo>,
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

        let tbl_typedef = &data.tbl_typedef;

        for ind in 0..tbl_typedef.row{

            let type_def = tbl_typedef.create_runtime_type_by_ind(ind as usize);
            self.info_types.push(type_def);
        }

        println!("{:?}",&self.info_types);

        self.dll = Some(dll.clone());
    }


    pub fn get_type_info(&self,name:&str,namespace:Option<&str>)->Option<&TypeInfo>{
        let type_vec = &self.info_types;

        let len = type_vec.len();

        let mut ret= None;
        for ind in 0..len {
            let item = &type_vec[ind];
            if item.name.as_ref() == name {
                if namespace.is_some() {
                    if item.namespace.as_ref() == namespace.unwrap() {
                        ret = Some(item);
                        break;
                    }
                }else{
                    ret = Some(item);
                    break;
                }
            }
        };
        ret
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

#[derive(Debug)]
pub struct TypeInfo{
    pub name:Rc<String>,
    pub namespace:Rc<String>,

    meta_index:usize,
}


impl RuntimeInfoType<TypeInfo,MetaTypeDef> for TypeInfo{
    fn new(meta: &MetaTypeDef, index: usize) -> TypeInfo {
        TypeInfo{
            name: Rc::clone(&meta.name),
            namespace: Rc::clone(&meta.namespace),
            meta_index: index,
        }
    }
}
