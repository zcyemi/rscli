#![allow(non_camel_case_types)]

use std::intrinsics::transmute;
use std::mem::size_of;
use std::fmt;

use crate::reader::BinaryReader;

#[derive(Copy, Clone, Debug)]
pub enum OpCode {
    nop = 0x0,
    ldarg_0 = 0x02,
    ldarg_1 = 0x03,
    add = 0x58,
    ldc_i4 = 0x20,
    stloc_0 = 0x0A,
    br_s = 0x2B,
    ldloc_0 = 0x06,
    ldloc_1 = 0x07,
    ret = 0x2A,
    call = 0x28,
}

impl From<u8> for OpCode {
    fn from(v: u8) -> OpCode {
        unsafe { transmute(v) }
    }
}


#[derive(Clone,Copy)]
#[repr(C)]
pub union Data {
    pub i8: i8,
    pub i16: i16,
    pub i32: i32,
    pub i64: i64,
    pub u8: u8,
    pub u16: u16,
    pub u32: u32,
    pub u64: u64,
    pub f32: f32,
    pub f64: f64,
    pub bool: bool,
    pub data_ref:DataRefType,
}

impl Data{
    #[inline]
    pub fn to_i8(&self)->i8{
        unsafe {self.i8}
    }
    #[inline]
    pub fn to_i16(&self)->i16{
        unsafe {self.i16}
    }
    #[inline]
    pub fn to_i32(&self)->i32{
        unsafe {self.i32}
    }
    #[inline]
    pub fn to_i64(&self)->i64{
        unsafe {self.i64}
    }
}


impl fmt::Debug for Data{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const SIZE:usize = size_of::<Data>();
        let ret:[u8;SIZE] = unsafe{ transmute::<&Self,[u8; SIZE]>(&self)};
        write!(f,"{:?}",ret)
    }
}


#[derive(Debug,Copy, Clone)]
pub enum DataRefType{
    none,
    type_token,
    call_site_descr,
    class,
    this_type,
    field,
    ctor,
    alignment
}

impl Data {
    #[inline]
    pub const fn none() -> Data {
        Data { data_ref:DataRefType::none }
    }
}

#[derive(Debug)]
pub struct Instruction {
    pub op: OpCode,
    pub data: Data,
}


pub fn parse_il_instructions(reader: &mut BinaryReader, count: u32) -> ( Vec<Instruction>,u8) {
    let mut set = Vec::new();
    let pos_max = reader.pos + count as usize;

    let mut param_list_len:u8 = 0;
    while reader.pos < pos_max {
        let code = reader.le_u8();
        let op = OpCode::from(code);
        let instruction = match op {
            OpCode::nop => Instruction { op, data: Data::none() },
            OpCode::ldc_i4 => Instruction { op, data: Data { i32: reader.le_i32() } },
            OpCode::stloc_0 => Instruction { op, data: Data::none() },
            OpCode::br_s => Instruction { op, data: Data { i8: reader.le_i8() } },
            OpCode::ldloc_0 => Instruction { op, data: Data::none() },
            OpCode::ret => Instruction { op, data: Data::none() },
            OpCode::call => {
                let (_tag, tbl_ind) = reader.tag_index();
                Instruction { op, data: Data { i32: tbl_ind as i32 } }
            }
            OpCode::ldarg_0 =>{
                param_list_len = 1;
                Instruction { op, data: Data::none() }
            },
            OpCode::ldarg_1 => {
                param_list_len = 2;
                Instruction { op, data: Data::none() }
            },
            OpCode::add => Instruction { op, data: Data::none() },
            _ => Instruction { op, data: Data::none() },
        };
        set.push(instruction);
    }
    (set,param_list_len)
}