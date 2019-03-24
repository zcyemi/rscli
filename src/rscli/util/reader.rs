#![allow(dead_code)]

use std::str;
use std::intrinsics::transmute;

#[derive(Debug,Default)]
pub struct DataPointer{
    pub rva: u32,
    pub size:u32,
}

impl DataPointer{
    pub fn default()->DataPointer{
        DataPointer{
            rva: 0,
            size: 0,
        }
    }
}

pub struct BinaryReader<'a> {
    pub raw_data: &'a [u8],
    pub pos: usize,
}

impl<'a> BinaryReader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        BinaryReader {
            raw_data: data,
            pos: 0,
        }
    }

    pub fn le_u8(self: &mut Self) -> u8 {
        let ret = self.raw_data[self.pos];
        self.pos += 1;
        ret
    }


    pub fn le_u16(self: &mut Self) -> u16 {
        let dat = self.raw_data;
        let pos = self.pos;
        let ret = ((dat[pos + 1] as u16) << 8) + dat[pos] as u16;
        self.pos += 2;
        ret
    }

    pub fn le_u32(self: &mut Self) -> u32 {
        let dat = self.raw_data;
        let pos = self.pos;
        let ret = ((dat[pos + 3] as u32) << 24) + ((dat[pos + 2] as u32) << 16) + ((dat[pos + 1] as u32) << 8) + dat[pos] as u32;
        self.pos += 4;
        ret
    }

    pub fn le_u64(&mut self)-> u64{
        let i = self.raw_data;
        let pos = self.pos;
        let ret = ((i[pos+7] as u64) << 56) + ((i[pos+6] as u64) << 48) + ((i[pos+5] as u64) << 40) + ((i[pos+4] as u64) << 32) + ((i[pos+3] as u64) << 24)
            + ((i[pos+2] as u64) << 16) + ((i[pos+1] as u64) << 8) + i[pos+0] as u64;
        self.pos +=8;
        ret
    }

    pub fn le_i8(&mut self)->i8{
        let ret = self.raw_data[self.pos];
        self.pos +=1;
        unsafe {transmute(ret)}
    }

    pub fn le_i32(&mut self)->i32{
        let pos = self.pos;
        let dat = self.raw_data;
        let ret = unsafe{ transmute::<[u8;4],i32>([dat[pos],dat[pos+1],dat[pos+2],dat[pos+3]])};
        self.pos+=4;
        ret
    }

    pub fn ate(self: &mut Self, bytes: usize) {
        self.pos += bytes;
    }

    pub fn le_uint(self:&mut Self,byte:u8)->u32{
        if byte == 2 {
            self.le_u16() as u32
        }else{
            self.le_u32()
        }
    }

    pub fn ate_till_tag(&mut self, tags: &[u8]) -> usize {
        let mut match_count = 0;
        let tag_count = tags.len();
        let mut pos = self.pos;
        ;
        let dat = self.raw_data;
        let dat_count = dat.len();
        let mut suc = false;
        while pos < dat_count {
            if dat[pos] == tags[match_count] {
                match_count += 1;
            } else {
                match_count = 0;
            }
            pos += 1;
            if match_count == tag_count {
                suc = true;
                break;
            }
        };
        let ret = if suc {
            pos = pos - tag_count;
            let ret = pos - self.pos;
            self.pos = pos;
            ret
        } else {
            0
        };
        ret
    }

    pub fn ate_till_byte(&mut self, byte: u8) -> usize {
        let mut pos = self.pos;
        let dat = self.raw_data;
        let dat_count = dat.len();

        let mut suc = false;
        while pos < dat_count {
            if dat[pos] == byte {
                suc = true;
                break;
            }
            pos += 1;
        };
        if suc {
            let ret = pos - self.pos;
            self.pos = pos;
            ret
        } else {
            0
        }
    }

    pub fn str(self: &mut Self, bytes: usize) -> String {
        let dat:&[u8] = self.raw_data;
        let pos = self.pos;
        let npos = pos + bytes;
        let ret:& [u8] = &dat[pos..npos];
        self.pos = npos;

        let ret:&str = match str::from_utf8(ret) {
            Ok(v)=>{v},
            Err(e)=>panic!("parse string failed"),
        };
        String::from(ret)
    }

    pub fn str_read(self:&mut Self) ->Option<String>{
        let data = self.raw_data;
        let mut pos = self.pos;
        let ret= if data[pos] == 0 {
            Option::None
        }else{
            pos +=1;
            let data_count = data.len();
            while pos < data_count {
                if data[pos] == 0{
                    break;
                }
                pos +=1;
            };
            let str = unsafe{str::from_utf8_unchecked(&data[self.pos..pos])};
            self.pos = pos+1;
            Some(String::from(str))
        };
        ret
    }

    pub fn str_pad(self: & mut Self) -> String {
        let dat = self.raw_data;
        let mut pos = self.pos;

        let dat_count = dat.len();

        while pos < dat_count {
            if dat[pos] == 0 {
                break;
            }
            pos += 1;
        }
        let byte_len = pos - self.pos;
        let str = unsafe{str::from_utf8_unchecked(&dat[self.pos..pos])};
        let pos_offset = (3- byte_len %4) %4;
        self.pos = pos + pos_offset + 1;

        String::from(str)
    }

    pub fn data_pointer(&mut self)->DataPointer{
        let rva = self.le_u32();
        let size = self.le_u32();
        DataPointer{
            rva:rva,
            size:size
        }
    }

    pub fn tag(self: &mut Self, tags: &[u8]) -> bool {
        let dat = self.raw_data;
        let pos = self.pos;
        let tag_len = tags.len();

        let mut suc = true;
        for t in 0..tag_len {
            if &dat[pos + t] != &tags[t] {
                suc = false;
                break;
            }
        };
        if suc {
            self.pos += tag_len;
        }
        suc
    }

    pub fn tag_panic(self: &mut Self, tags: &[u8]) {
        if !self.tag(tags) {
            panic!("can not read tags: {:?}", tags);
        }
    }

    pub fn seek(self: &mut Self, pos: usize) {
        (*self).pos = pos;
    }

    pub fn repeat<T>(&mut self, f: fn(&mut Self) -> T, count: u32) -> Vec<T> {
        let mut ret: Vec<T> = Vec::new();
        for _ in 0..count {
            let v = f(self);
            ret.push(v);
        }
        ret
    }
}