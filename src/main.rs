#[macro_use]
extern crate nom;

mod rscil;

//use rscil::runtime;
use rscil::loader;

fn main() {

//    let r = runtime::StackMachine::new();
//    r.test();


    loader::loader_test();

}
