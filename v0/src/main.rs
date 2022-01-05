pub mod vm;

use vm::Op::*;

fn main() {
    let mut vm = vm::VM::new();
    vm.act(PushI64(2));
    vm.act(PushI64(2));
    vm.act(AddI64(0, 1)); //result of addition is pushed to the top of the stack
    assert!(vm.stack[2] == 4);

    vm.act(PushI64(-8));
    vm.act(PushI64(-20));
    vm.act(AddI64(3, 4));
    assert!(vm.stack[5] as i64 == -28);

    vm.act(AddI64(3, 0));
    assert!(vm.stack[6] as i64 == -6);
    println!("{}", vm.frame_height());
    vm.act(PushStackFrame);
    println!("{}", vm.frame_height());
}

#[test]
fn new_vm_is_empty(){
    let vm = vm::VM::new();
    assert!(vm.stack_len() == 0);
}

#[test]
fn pass_does_nothing(){
    let mut vm = vm::VM::new();
    vm.act(Pass);
    assert!(vm.stack_len() == 0);
}

#[test]
fn push_increments_len(){
    let mut vm = vm::VM::new();
    assert!(vm.stack_len() == 0);
    vm.act(PushI64(4));
    assert!(vm.stack_len() == 1);
    vm.act(PushI64(-12));
    assert!(vm.stack_len() == 2);
}

#[test]
fn i64_add_works(){
    let mut vm = vm::VM::new();
    vm.act(PushI64(2));
    vm.act(PushI64(2));
    vm.act(AddI64(0, 1)); //result of addition is pushed to the top of the stack
    assert!(vm.stack[2] == 4);

    vm.act(PushI64(-8));
    vm.act(PushI64(-20));
    vm.act(AddI64(3, 4));
    assert!(vm.stack[5] as i64 == -28);

    vm.act(AddI64(3, 0));
    assert!(vm.stack[6] as i64 == -6);
}

#[test]
fn pushing_stack_frame_makes_everything_inaccessable(){
    let mut vm = vm::VM::new();
    vm.act(PushI64(2));
    vm.act(PushI64(2));
    vm.act(AddI64(0, 1)); //result of addition is pushed to the top of the stack
    assert!(vm.stack[2] == 4);

    vm.act(PushI64(-8));
    vm.act(PushI64(-20));
    vm.act(AddI64(3, 4));
    assert!(vm.stack[5] as i64 == -28);

    vm.act(AddI64(3, 0));
    assert!(vm.stack[6] as i64 == -6);

    vm.act(PushStackFrame);
    assert!(vm.get(0) == 0);
}


