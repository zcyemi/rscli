
pub mod reader;


pub struct BitUtility{
}

impl BitUtility{
    pub fn bits_count(v:u8)->u8{
        let mut x =v;
        let mut c:u8 = 0;
        while x > 0 {
            x &= x-1;
            c = c+1;
        };
        c
    }

    pub fn bits_count_u64(v:u64)->u8{
        let mut x =v;
        let mut c:u8 = 0;
        while x > 0 {
            x &= x-1;
            c = c+1;
        };
        c
    }


}
