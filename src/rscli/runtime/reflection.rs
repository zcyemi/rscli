use std::rc::Rc;
use std::cell::RefCell;
use crate::rscli::loader::DllFile;
use crate::rscli::meta::CLIData;
use crate::rscli::meta::tbl::*;
use std::fs::OpenOptions;
use crate::rscli::meta::tbl::CLITableId::TypeDef;
use core::borrow::BorrowMut;


#[derive(Default)]
pub struct ReflectionInfo {
    dll: Rc<RefCell<DllFile>>,

    info_class: Vec<Rc<ClassInfo>>,
    info_method: Vec<Rc<MethodInfo>>,
    info_assembly: Vec<Rc<AssemblyInfo>>,
}

impl ReflectionInfo {
    pub fn new() -> ReflectionInfo {
        Default::default()
    }


    pub fn load_dll(&mut self, dll: &Rc<RefCell<DllFile>>) {
        //process dll
//        let mut data = &dll.as_ref().borrow_mut().clidata;
//        let filter = |x: &MetaAssembly| x.name.as_ref() == "netdlltest";

//        let asm: Option<AssemblyInfo> = data.tbl_assembly.create_runtime_type_by_filter(&filter);
//        match asm {
//            Some(info) => self.info_assembly.push(info),
//            None => (),
//        }
        self.dll = Rc::clone(dll);
    }

    pub fn get_assembly(&mut self, assembly_name: &str) -> Option<Rc<AssemblyInfo>> {
        let dll = self.dll.as_ref().borrow();
        let tbl_assembly = &dll.clidata.tbl_assembly;
        let mut index = 0_usize;
        let asm = tbl_assembly.get_data_by_filter_ind(&(|x| x.name.as_ref() == assembly_name), &mut index);
        if asm.is_some() {
            let asm_info = Rc::new(AssemblyInfo::new(&asm.unwrap(), index));
            let ret = asm_info.clone();
            self.info_assembly.push(asm_info);
            Some(ret)
        } else {
            Option::None
        }
    }

    pub fn get_class_info(&mut self, class_name: &str) -> Option<Rc<ClassInfo>> {
        let dll = self.dll.as_ref().borrow();
        let clidata = &dll.clidata;

        let tbl_typedef = &clidata.tbl_typedef;

        let mut index = 0_usize;
        let typedef = tbl_typedef.get_data_by_filter_ind(&(|x| x.name.as_ref() == class_name), &mut index);

        if typedef.is_none() {
            Option::None
        } else {
            let typedef = typedef.unwrap();

            let method_list_start = typedef.method_list as usize - 1;
            let total_rows = tbl_typedef.row as usize;

            let mut method_list_end = method_list_start;
            if index == total_rows - 1 {
                method_list_end = clidata.tbl_methoddef.row as usize;
            } else {
                let next_typedef = tbl_typedef.get_data_by_index(index + 1);
                method_list_end = next_typedef.method_list as usize;
            }

            let methods = self.get_method_info_by_index_range(method_list_start, method_list_end);
            for item in &methods {
                self.info_method.push(Rc::clone(&item));
            }
            let class_info = ClassInfo::new(&typedef, index, methods);

            let rc = Rc::new(class_info);
            self.info_class.push(rc.clone());
            Some(rc)
        }
    }

    fn get_method_info_by_index_range(&self, start: usize, end: usize) -> Vec<Rc<MethodInfo>> {
        let dll = self.dll.as_ref().borrow();
        let clidata = &dll.clidata;
        let tbl_method = &clidata.tbl_methoddef;
        let mut vec = Vec::new();

        for ind in start..end {
            let method = tbl_method.get_data_by_index(ind);
            let method_info = MethodInfo::new(method, ind);
            let rc = Rc::new(method_info);
            vec.push(rc);
        }
        vec
    }

    pub fn get_method_info(&self, method_name: &str, class_info: &Rc<ClassInfo>) -> Option<Rc<MethodInfo>> {
        let class = class_info.as_ref();
        let mut ret = None;
        for method in &class.methods {
            if method.name.as_ref() == method_name {
                ret = Some(method.clone());
                break;
            }
        }
        ret
    }
}

#[derive(Debug)]
pub struct AssemblyInfo {
    pub name: Rc<String>,

    pub meta_index: usize,
}

impl AssemblyInfo {
    pub fn new(meta: &MetaAssembly, index: usize) -> AssemblyInfo {
        AssemblyInfo {
            name: meta.name.clone(),
            meta_index: index,
        }
    }
}

#[derive(Debug)]
pub struct ClassInfo {
    pub name: Rc<String>,
    pub namespace: Rc<String>,
    pub meta_index: usize,
    pub methods: Vec<Rc<MethodInfo>>,
}

impl ClassInfo {
    pub fn new(meta: &MetaTypeDef, index: usize, method_list: Vec<Rc<MethodInfo>>) -> ClassInfo {
        ClassInfo {
            name: meta.name.clone(),
            namespace: meta.namespace.clone(),
            meta_index: index,
            methods: method_list,
        }
    }
}

#[derive(Debug)]
pub struct MethodInfo {
    pub name: Rc<String>,
    pub meta_index: usize,
}

impl MethodInfo {
    pub fn new(meta: &MetaMethodDef, index: usize) -> MethodInfo {
        MethodInfo {
            name: meta.name.clone(),
            meta_index: index,
        }
    }
}