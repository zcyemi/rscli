#![allow(dead_code)]
#![feature(const_raw_ptr_to_usize_cast)]
#![feature(core_intrinsics)]
#![feature(untagged_unions)]

#[macro_use]
extern crate lazy_static;

mod rscli;
use rscli::util::reader::BinaryReader;
use std::rc::Rc;
use std::cell::RefCell;
use crate::rscli::runtime::context::Context;
use crate::rscli::runtime::il::Data;

fn main() {

    let dll_path = "DLL_PATH";
    let dll = rscli::loader::load_dll(dll_path);
    let rc_dll = Rc::new(RefCell::new(dll));

    let mut context = Context::new();
    context.reflection.load_dll(&rc_dll);

    let test_class = context.reflection.get_class_info(&"Class1").unwrap();
    let method_add = context.reflection.get_method_info(&"add",&test_class).unwrap();
    let ret = context.exec(&method_add,Some(vec![Data{i32:1574},Data{i32:-433}]));
    println!("{:?}",&ret.unwrap().to_i32());



}



