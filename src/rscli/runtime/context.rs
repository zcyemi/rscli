#![allow(non_camel_case_types)]

use std::rc::Rc;
use std::cell::{RefCell, UnsafeCell};
use crate::rscli::loader::DllFile;
use crate::rscli::runtime::reflection::*;

use crate::rscli::runtime::il::*;

use std::mem::transmute;

pub struct Context {
    pub reflection: ReflectionInfo,

}

impl Context {
    pub fn new() -> Context {
        Context {
            reflection: ReflectionInfo::new()
        }
    }

    pub fn exec(&self, method_info: &MethodInfo,args:Option<Vec<OpData>>) -> Option<OpData> {
        let mut stack: ExecStack = Default::default();

        let inst = &method_info.instruction.borrow().instruction;
        stack.exec(inst,args)
    }
}

#[derive(Debug)]
pub struct ExecStack {
    pub stack: Vec<OpData>,
    pub local:[OpData;8],
}

impl Default for ExecStack{
    fn default() -> Self {
        ExecStack{
            stack:Vec::new(),
            local:[OpData::none;8],
        }
    }
}

impl ExecStack {
    pub fn exec(&mut self, instructions: &Vec<Instruction>,args:Option<Vec<OpData>>) -> Option<OpData> {
        //TODO: need to check the args is match method parameters

        let il_count = instructions.len();

        let stack = &mut self.stack;

        let mut ret = Option::None;

        let args = match  args {
            Some(v)=>v,
            None=>Vec::new()
        };

        for mut t in 0..il_count {
            let il = &instructions[t];
            let op = il.op;
            match op {
                OpCode::nop => (),
                OpCode::ldc_i4 => {
                    stack.push(il.data.clone());
                }
                OpCode::stloc_0=>{
                    let val = stack.pop().unwrap();
                    self.local[0] = val;
                }
                OpCode::ldloc_0=>{
                    stack.push(self.local[0]);
                }
                OpCode::br_s=>{
                    let ptr:*const i8 = unsafe{ transmute(&il.data)};
                    let v:i8 = unsafe{*ptr.offset(1)};
                    t += v as usize;
                }
                OpCode::ret => {
                    ret = stack.pop();
                }
                OpCode::ldarg_0 =>{
                    stack.push(args[0]);
                }
                OpCode::ldarg_1=>{
                    stack.push(args[1]);
                }
                OpCode::add=>{
                    //TODO: currently treat arguments as i32
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                }
                _ => (),
            }
        }
        ret
    }
}