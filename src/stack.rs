pub struct StackPointer{
    array: [u16; 16],
    length: usize
}

impl StackPointer {
    pub fn new() -> StackPointer{
        StackPointer {
            array: [0; 16],
            length: 0
        }
    }

    pub fn push(&mut self, addr: u16) -> Result<(), String>{
        if self.length == 16 { return Err(String::from("Stack Overflow")) }

        self.array[self.length] = addr;
        self.length += 1;

        Ok(())
    }

    pub fn pop(&mut self) -> Option<u16>{
        if self.length == 0 { return None }

        let result: u16 = self.array[self.length - 1];

        self.array[self.length - 1] = 0;
        self.length -= 1;

        Some(result)
    }
}