pub struct VM{
    pub stack: Vec<usize>,
    pub base: usize,
}

impl VM{
    pub fn new() -> VM{
        return VM{ 
            stack: Vec::new(),
            base: 0,
        };
    }
    pub fn act(&mut self, op: Op){
        use Op::*;
        match op {
            Pass => (),
            PushI64(val) => {
                self.stack.push(val as usize);
            }
            AddI64(index1, index2) => {
                let num1 = self.stack[index1] as i64;
                let num2 = self.stack[index2] as i64;
                let ans = num1 + num2;
                self.act(PushI64(ans)); //always use a push method if you want to add to the top of the stack
            }
            PushStackFrame => {
                self.stack.push(self.base);
                self.base = self.frame_height();
            }
            _ => ()
        }
    }
    pub fn stack_len(&self) -> usize{
        self.stack.len()
    }
    pub fn get(&self, index: usize) -> usize{
        self.stack[index + self.base]
    }
    pub fn frame_height(&self) -> usize{
        self.stack_len() - self.base
    }

}

pub enum Op{  //Op short for "operation"
    Pass,

    PushI64(i64),
    /*
    all the mathmatical operations will rely on having pointers to thier arguments. 
    all the 'usize' elements will be treated as pointers (if rust can do that in unsafe blocks).

    For now, they're indecies onto the stack.
    */
    AddI64(usize, usize),
    SubI64(usize, usize),
    MulI64(usize, usize),
    DivI64(usize, usize),

    PushStackFrame,

    CallFnName(String),
    CallFnPtr(usize)
}