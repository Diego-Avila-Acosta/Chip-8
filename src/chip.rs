use crate::stack::StackPointer;
use crate::timer::Timer;
use super::rom::Rom;
use rand::prelude::*;

const SPRITES: [[u8;5]; 16] = [
    [0xF0, 0x90, 0x90, 0x90, 0xF0], // 0
    [0x20, 0x60, 0x20, 0x20, 0x70], // 1
    [0xF0, 0x10, 0xF0, 0x80, 0xF0], // 2
    [0xF0, 0x10, 0xF0, 0x10, 0xF0], // 3
    [0x90, 0x90, 0xF0, 0x10, 0x10], // 4
    [0xF0, 0x80, 0xF0, 0x10, 0xF0], // 5
    [0xF0, 0x80, 0xF0, 0x90, 0xF0], // 6
    [0xF0, 0x10, 0x20, 0x40, 0x40], // 7
    [0xF0, 0x90, 0xF0, 0x90, 0xF0], // 8
    [0xF0, 0x90, 0xF0, 0x10, 0xF0], // 9
    [0xF0, 0x90, 0xF0, 0x90, 0x90], // A
    [0xE0, 0x90, 0xE0, 0x90, 0xE0], // B
    [0xF0, 0x80, 0x80, 0x80, 0xF0], // C
    [0xE0, 0x90, 0x90, 0x90, 0xE0], // D
    [0xF0, 0x80, 0xF0, 0x80, 0xF0], // E
    [0xF0, 0x80, 0xF0, 0x80, 0x80], // F
];

pub struct Chip8 {
    registers: [u8; 16],
    i_register: u16,
    delay_timer: Timer,
    sound_timer: Timer,
    memory: [u8; 4096],
    pc: usize,
    sp: StackPointer,
    pub display: [u64; 32]
}

impl Chip8 {
    pub fn new(rom: Rom) -> Chip8{
        let mut memory: [u8; 4096] = [0; 4096];
        
        let mut i = 0;
        for sprite in SPRITES {
            for byte in sprite {
                memory[i] = byte;
                i += 1;
            }
        }

        for (i, addr) in (0x200..(rom.length + 0x200)).enumerate(){
            memory[addr] = rom.program[i];
        }

        Chip8 {
            registers: [0; 16],
            i_register: 0,
            delay_timer: Timer::new(60),
            sound_timer: Timer::new(60),
            memory,
            pc: 0x200,
            sp: StackPointer::new(),
            display: [0; 32]
        }
    }

    pub fn run_instruction(&mut self, delta_time: f64, key_pressed: Option<u8>) -> bool {
        self.delay_timer.check(delta_time);

        let instruction = ((self.memory[self.pc] as u16) << 8) + self.memory[self.pc + 1] as u16;
        self.pc += 2;
        
        if instruction == 0x0000 { return true; }
        false
    }

}