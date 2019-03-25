#![allow(non_camel_case_types)]

use std::rc::Rc;
use std::cell::{RefCell, UnsafeCell};
use crate::rscli::loader::DllFile;
use crate::rscli::runtime::reflection::*;

use crate::rscli::runtime::il::*;
use std::intrinsics::transmute;

pub struct Context {
    pub reflection: ReflectionInfo,

}

impl Context {
    pub fn new() -> Context {
        Context {
            reflection: ReflectionInfo::new()
        }
    }

    pub fn exec(&self, method_info: &MethodInfo) -> Option<OpData> {
        let mut stack: ExecStack = Default::default();

        let inst = &method_info.instruction.borrow().instruction;
        stack.exec(inst)
    }
}

#[derive(Default)]
pub struct ExecStack {
    pub stack: Vec<OpData>,
}

impl ExecStack {
    pub fn exec(&mut self, instructions: &Vec<Instruction>) -> Option<OpData> {
        let il_count = instructions.len();

        let stack = &mut self.stack;

        let mut ret = Option::None;

        for t in 0..il_count {
            let il = &instructions[t];
            let op = il.op;
            match op {
                OpCode::nop => (),
                OpCode::ldc_i4 => {
                    stack.push(il.data.clone());
                }
                OpCode::ret => {
                    ret = stack.pop();
                }
                _ => (),
            }
        }
        ret
    }
}