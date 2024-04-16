pub struct StackPointer{
    array: [usize; 16],
    length: usize
}

impl StackPointer {
    pub fn new() -> StackPointer{
        StackPointer {
            array: [0; 16],
            length: 0
        }
    }

    pub fn push(&mut self, addr: usize){
        if self.length == 16 { panic!("Stack Overflow") }

        self.array[self.length] = addr;
        self.length += 1;
    }

    pub fn pop(&mut self) -> usize{
        if self.length == 0 { panic!("The stack pointer is empty") }

        let result: usize = self.array[self.length - 1];

        self.array[self.length - 1] = 0;
        self.length -= 1;

        result
    }
}