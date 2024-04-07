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

    pub fn push(&mut self, addr: usize) -> Result<(), String>{
        if self.length == 16 { return Err(String::from("Stack Overflow")) }

        self.array[self.length] = addr;
        self.length += 1;

        Ok(())
    }

    pub fn pop(&mut self) -> Option<usize>{
        if self.length == 0 { return None }

        let result: usize = self.array[self.length - 1];

        self.array[self.length - 1] = 0;
        self.length -= 1;

        Some(result)
    }
}