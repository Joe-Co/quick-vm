pub mod vm;

use vm::Op::*;

fn main() {
    let mut vm2 = vm::VM::new();
    vm2.act(vm::Op::Pass);
    vm2.act(PushI32(12));
    vm2.act(PushI64(-32));
    vm2.act(AddI64(0, 1));
    let last_index = vm2.num_items - 1;
    println!("{}", vm2.stack[last_index]); 
}



