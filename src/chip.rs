use crate::stack::StackPointer;
pub struct Chip8 {
    registers: [u8; 16],
    i_register: u16,
    delay_timer: u16,
    sound_timer: u16,
    memory: [u8; 4096],
    pc: u16,
    sp: StackPointer,
}

impl Chip8 {
    fn new() -> Chip8{
        Chip8 {
            registers: [0; 16],
            i_register: 0,
            delay_timer: 0,
            sound_timer: 0,
            memory: [0; 4096],
            pc: 0,
            sp: StackPointer::new()
        }
    }
}