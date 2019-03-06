mod rscil;

use rscil::runtime;

fn main() {

    let r = runtime::StackMachine::new();
    r.test();
}
