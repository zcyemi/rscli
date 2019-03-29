#![allow(non_camel_case_types)]

use std::mem::transmute;

use crate::il::*;
use crate::reflection::*;

pub struct Context {
    pub reflection: ReflectionInfo,

}

impl Context {
    pub fn new() -> Context {
        Context {
            reflection: ReflectionInfo::new()
        }
    }

    pub fn exec(&self, method_info: &MethodInfo,args:Option<Vec<Data>>) -> Option<Data> {
        let mut stack: ExecStack = Default::default();

        let inst = &method_info.instruction.borrow().instruction;
        stack.exec(inst,args)
    }
}

#[derive(Debug)]
pub struct ExecStack {
    pub stack: Vec<Data>,
    pub local:[Data;8],
}

impl Default for ExecStack{
    fn default() -> Self {
        ExecStack{
            stack:Vec::new(),
            local:[Data::none();8],
        }
    }
}

impl ExecStack {
    #[allow(unused_assignments)]
    pub fn exec(&mut self, instructions: &Vec<Instruction>,args:Option<Vec<Data>>) -> Option<Data> {
        //TODO: need to check the args is match method parameters

        let il_count = instructions.len();

        let stack = &mut self.stack;

        let mut ret:Option<Data> = None;

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
                    stack.push(il.data);
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
                    let a = stack.pop().unwrap().to_i32();
                    let b = stack.pop().unwrap().to_i32();
                    stack.push(Data{i32:a+b});
                }
                _ => (),
            }
        }
        ret
    }



}