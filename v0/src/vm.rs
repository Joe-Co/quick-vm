pub struct VM{
    pub map: i32 // dummy type cuz idk how to do hashmaps in rust yet
}

impl VM{
    pub fn new() -> VM{
        return VM{ map: 0};
    }
    pub fn act(&mut self, op: Op){
        //do nothing!
    }
}

pub enum Op{
    Pass,
    RaiseStack(i32),

    //all the mathmatical operations will rely on having pointers to thier arguments. all the 'usize' elements will be treated as pointers (if rust can do that in unsafe blocks)
    AddI32(usize, usize),
    SubI32(usize, usize),
    MulI32(usize, usize),
    DivI32(usize, usize),

    CallFnName(String),
    CallFnPtr(usize)
}