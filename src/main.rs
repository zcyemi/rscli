#![allow(dead_code)]

#![feature(core_intrinsics)]

#[macro_use]
extern crate lazy_static;

mod rscli;

use rscli::util::reader::BinaryReader;

use rscli::runtime::*;
use std::rc::Rc;
use std::cell::RefCell;
use crate::rscli::runtime::context::Context;
use core::borrow::BorrowMut;
use std::mem::size_of;
use std::intrinsics::{size_of_val, transmute};

fn main() {
    let dll = rscli::loader::load_dll("D:/TestDll.dll");
    let rc_dll = Rc::new(RefCell::new(dll));

    let mut context = Context::new();
    context.reflection.load_dll(&rc_dll);

    let test_class = context.reflection.get_class_info(&"TestClass").unwrap();
    let method_get_num = context.reflection.get_method_info(&"getNum", &test_class).unwrap();

    println!("method: {:?}",& method_get_num);

    let ret= context.exec(&method_get_num);
    println!("{:?}",&ret);

}

