pub struct VM{
    pub map: i32, // dummy type cuz idk how to do hashmaps in rust yet
    pub stack: Vec<usize>,
    pub topptr: i32,
    pub num_items: usize,
}

impl VM{
    pub fn new() -> VM{
        return VM{ 
            map: 0,
            stack: Vec::new(),
            num_items: 0,
            topptr: 0,
        };
    }
    pub fn act(&mut self, op: Op){
        use Op::*;
        match op {
            Pass => (),
            PushI32(val) => {  //todo figure out how to make this thing work for 32 bit things seperatly
                self.stack.push(val as usize);
                self.num_items += 1; //why can't I do num_items++??
            }
            PushI64(val) => {
                self.stack.push(val as usize);
                self.num_items += 1; //why can't I do num_items++??
            }
            AddI64(index1, index2) => {
                let num1 = self.stack[index1] as i64;
                let num2 = self.stack[index2] as i64;
                let ans = num1 + num2;
                self.act(PushI64(ans)); //always use a push method if you want to add to the top of the stack
            }
            _ => ()
        }
    }

}

pub enum Op{  //Op short for "operation"
    Pass,

    PushI32(i32),
    PushI64(i64),
    /*
    all the mathmatical operations will rely on having pointers to thier arguments. 
    all the 'usize' elements will be treated as pointers (if rust can do that in unsafe blocks)
    */
    AddI64(usize, usize),
    SubI32(usize, usize),
    MulI32(usize, usize),
    DivI32(usize, usize),

    CallFnName(String),
    CallFnPtr(usize)
}