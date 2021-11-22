pub mod vm;

fn main() {
    let vm1 = vm::VM{
        map: 4
    };
    let mut vm2 = vm::VM::new();
    vm2.act(vm::Op::Pass);

    println!("{}, {}", 0, vm2.map);
}



