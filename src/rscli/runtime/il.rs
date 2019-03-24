#![allow(non_camel_case_types)]

use crate::rscli::util::reader::BinaryReader;

pub enum OpCode {
    nop = 0x0,
    ldc_i4 = 0x1A,
    stdloc_0 = 0x0A,
    br_s = 0x2B,
    ldloc_0 = 0x06,
    ldloc_1 = 0x07,
    ret = 0x2A,
}

pub enum OpData {
    none,
    int8(i8),
    int32(i32),
    type_token,
    method,
    call_site_descr,
    class,
    this_type,
    int64(i64),
    float32(f32),
    float64(f64),
    field,
    string,
    ctor,
    alignment,
}

pub struct Instruction{
    pub op: OpCode,
    pub data: OpData,
}


pub fn parse_il_instructions(reader: &mut BinaryReader)->Vec<Instruction>{
    Vec::new()
}