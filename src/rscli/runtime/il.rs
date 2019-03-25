#![allow(non_camel_case_types)]

use crate::rscli::util::reader::BinaryReader;
use std::intrinsics::transmute;

#[derive(Copy, Clone, Debug)]
pub enum OpCode {
    nop = 0x0,
    ldc_i4 = 0x20,
    stloc_0 = 0x0A,
    br_s = 0x2B,
    ldloc_0 = 0x06,
    ldloc_1 = 0x07,
    ret = 0x2A,
    ldarg_0 = 0x02,
    call = 0x28,
}

impl From<u8> for OpCode {
    fn from(v: u8) -> OpCode {
        unsafe { transmute(v) }
    }
}

#[derive(Debug,Copy, Clone)]
pub enum OpData {
    none,
    int8(i8),
    int16(i16),
    int32(i32),
    int64(i64),
    unt8(i8),
    unt16(u16),
    unt32(u32),
    unt64(u64),
    float32(f32),
    float64(f64),

    type_token,
    method,
    call_site_descr,
    class,
    this_type,
    field,
    string,
    ctor,
    alignment,
}

#[derive(Debug)]
pub struct Instruction {
    pub op: OpCode,
    pub data: OpData,
}


pub fn parse_il_instructions(reader: &mut BinaryReader, count: u32) -> Vec<Instruction> {
    let mut set = Vec::new();
    let pos_max = reader.pos + count as usize;
    while reader.pos < pos_max {
        let code = reader.le_u8();
        let op = OpCode::from(code);
        let instruction = match op {
            OpCode::nop => Instruction { op, data: OpData::none },
            OpCode::ldc_i4 => Instruction { op, data: OpData::int32(reader.le_i32()) },
            OpCode::stloc_0 => Instruction { op, data: OpData::none },
            OpCode::br_s => Instruction { op, data: OpData::int8(reader.le_i8()) },
            OpCode::ldloc_0 => Instruction { op, data: OpData::none },
            OpCode::ret => Instruction { op, data: OpData::none },
            OpCode::call => {
                let (tag,tbl_ind) = reader.tag_index();
                Instruction { op, data: OpData::int32(tbl_ind as i32) }
            },
            OpCode::ldarg_0 => Instruction { op, data: OpData::none },
            _ => Instruction { op, data: OpData::none },
        };
        set.push(instruction);
    }
    set
}