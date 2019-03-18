
#[macro_use]
extern crate  lazy_static;

mod rscli;

use rscli::util::reader::BinaryReader;

fn main() {

    rscli::loader::load_dll("D:/TestDll.dll");

}
