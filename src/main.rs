#![allow(dead_code)]
#[macro_use]
extern crate lazy_static;

mod rscli;

use rscli::util::reader::BinaryReader;

use rscli::runtime::*;
use std::rc::Rc;
use std::cell::RefCell;
use crate::rscli::runtime::context::Context;

fn main() {
    let dll = rscli::loader::load_dll("D:/TestDll.dll");

    let rc_dll = Rc::new(RefCell::new(dll));


    let mut context = Context::new();
    context.reflection.load_dll(&rc_dll);


    let type_x = context.reflection.get_type_info(&"TestClass",Some(&"TestLib"));

    println!("{:?}",type_x);
}
