
use std::rc::Rc;
use std::cell::RefCell;
use crate::rscli::loader::DllFile;
use crate::rscli::runtime::reflection::*;

pub struct Context{
    pub reflection:ReflectionInfo,

}

impl Context{

    pub fn new()->Context{
        Context{
            reflection:ReflectionInfo::new()
        }
    }


}