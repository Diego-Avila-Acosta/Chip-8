use core::panic;
use crate::stack::StackPointer;
use std::time::{Instant, Duration};
use std::thread::{sleep};

const PERIOD_DELAY_AS_SECS: f64 = 1.0 / 60.0;

pub struct Chip8 {
    registers: [u8; 16],
    i_register: u16,
    delay_timer: u16,
    sound_timer: u16,
    memory: [u8; 4096],
    pc: usize,
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

    fn execute_instruction(&mut self, instruction: u16){
        let bytes: [u8;2] = instruction.to_be_bytes();

        match bytes[0] {
            0x00 => {
                match bytes[1] {
                    0xE0 => {}, //Clear display
                    0xEE => { self.return_subroutine() },
                    _ => panic!("Incorrect instruction")
                }
            },
            0x10..=0x1F => { // Jump to address
                let addr: u16 = Chip8::subtract_instruction(0x10, bytes);
                self.jump_to_address(addr as usize);
            },
            0x20..=0x2F => { // Call subroutine
                let addr: u16 = Chip8::subtract_instruction(0x10, bytes);
                self.call_subroutine(addr as usize);
            },
            0x30..=0x3F => { // Skip if equal (1 register)
                let register = self.registers[(bytes[0] - 0x30) as usize];
                self.skip_if_equal(register, bytes[1]);
            },
            0x40..=0x4F => { // Skip if not equal (1 register)
                let register = self.registers[(bytes[0] - 0x40) as usize];
                self.skip_if_not_equal(register, bytes[1]);
            },
            0x50..=0x5F => { // Skip if equal (2 registers)
                let register = self.registers[(bytes[0] - 0x50) as usize];
                let register2 = self.registers[bytes[1] as usize];

                self.skip_if_equal(register, register2);
            },
            0x60..=0x6F => { // Set register
                let addr = (bytes[0] - 0x60) as usize;
                self.set_register(addr, bytes[1]);
            },
            0x70..=0x7F => { // Add value to register
                let addr = (bytes[0] - 0x70) as usize;
                self.add_to_register(addr, bytes[1]);
            },
            0x80..=0x8F => {
                match bytes[1] {
                    0x01..=0xF1 => {
                        let addr = (bytes[0] - 0x80) as usize;
                        let value = self.registers[(bytes[1] - 0x01) as usize];
                        self.bitwise_OR(addr, value);
                    },
                    0x02..=0xF2 => {
                        let addr = (bytes[0] - 0x80) as usize;
                        let value = self.registers[(bytes[1] - 0x02) as usize];
                        self.bitwise_AND(addr, value);
                    },
                    0x03..=0xF3 => {
                        let addr = (bytes[0] - 0x80) as usize;
                        let value = self.registers[(bytes[1] - 0x03) as usize];
                        self.bitwise_XOR(addr, value);
                    },
                    0x04..=0xF4 => {
                        let addr = (bytes[0] - 0x80) as usize;
                        let value = self.registers[(bytes[1] - 0x04) as usize];
                        self.add_registers(addr, value);
                    },
                    0x05..=0xF5 => {
                        let addr = (bytes[0] - 0x80) as usize;
                        let value = self.registers[(bytes[1] - 0x05) as usize];
                        self.subtract_registers(addr, value);
                    },
                    0x06..=0xF6 => {},
                    0x07..=0xF7 => {},
                    0x0E..=0xFE => {},
                    _ => todo!()
                }
            },
            0x90..=0x9F => { // Skip if not equal(2 registers)
                let register = self.registers[(bytes[0] - 0x50) as usize];
                let register2 = self.registers[bytes[1] as usize];

                self.skip_if_not_equal(register, register2);
            },
            0xA0..=0xAF => { // Set register I
                let value = Chip8::subtract_instruction(0xA0, bytes);

                self.i_register = value;
            },
            0xB0..=0xBF => { // Jump to address plus register
                let mut addr = Chip8::subtract_instruction(0xB0, bytes);

                addr += self.registers[0x0 as usize] as u16;

                self.jump_to_address(addr as usize);
            },
            0xC0..=0xCF => {}, // Random Byte
            0xD0..=0xDF => {}, // Display n-byte
            0xE0..=0xEF => {
                match bytes[1] {
                    0x9E => {}, // Skip instruction if key is pressed
                    0xA1 => {}, // Skip instruction if key is not pressed
                    _ => todo!()
                }
            },
            0xF0..=0xFF => {
                match bytes[1] {
                    0x07 => {}, // Set register to delay timer value
                    0x0A => {}, // Wait for a key press, and store key value in register
                    0x15 => {}, // Set delay timer to the value of a register
                    0x18 => {}, // Set sound timer to the value of a register
                    0x1E => {}, // Adds I and register x, and stores it in register x
                    0x29 => {}, // Set I = location of sprite for digit x
                    0x33 => {}, // 
                    0x55 => {}, // Store registers 0 through x in memory starting at location I
                    0x65 => {}, // Read registers 0 through x from memory starting at location I
                    _ => todo!()
                }
            },
            _ => todo!()
        }
    }

     fn subtract_instruction(instruction: u8, bytes: [u8;2]) -> u16{
        let byte: u8 = bytes[0] - instruction;
        let mut byte: u16 = byte.into();
        byte = byte << 8;

        byte + bytes[1] as u16
    }

    fn call_subroutine(&mut self, addr_subroutine: usize){
        match self.sp.push(self.pc) {
            Err(e) => panic!("{e}"),
            _ => ()
        }
        self.pc = addr_subroutine;
    }

    fn return_subroutine(&mut self){
        match self.sp.pop() {
            Some(addr) => self.pc = addr,
            None => panic!("Can't return if there is no address in the stack")
        };
    }

    fn jump_to_address(&mut self, addr: usize) {
        if addr > 4095 { panic!("Can't access memory out of bounds"); }

        self.pc = addr;
    }

    fn skip_if_equal(&mut self, b1: u8, b2: u8){
        if b1 == b2 { self.pc += 2; }
    }

    fn skip_if_not_equal(&mut self, b1: u8, b2: u8){
        if b1 != b2 { self.pc += 2; }
    }

    fn set_register(&mut self, addr: usize, value: u8){
        self.registers[addr] = value;
    }

    fn add_to_register(&mut self, addr: usize, value: u8){
        self.registers[addr] += value;
    }

    fn bitwise_OR(&mut self, addr: usize, value: u8) {
        self.registers[addr] |= value;
    }
    
    fn bitwise_AND(&mut self, addr: usize, value: u8) {
        self.registers[addr] &= value;
    }

    fn bitwise_XOR(&mut self, addr: usize, value: u8) {
        self.registers[addr] ^= value;
    }

    fn add_registers(&mut self, addr: usize, value: u8) {
        let (value, carry) = self.registers[addr].overflowing_add(value);
        self.registers[addr] = value;
        self.registers[0xF as usize] = if carry {1} else {0};
    }

    fn subtract_registers(&mut self, addr: usize, value: u8) {
        if self.registers[addr] < value { self.registers[0xF as usize] = 1 }
        else { self.registers[0xF as usize] = 0 }

        self.registers[addr] -= value;
    }

    fn delay(&mut self){
        while self.delay_timer != 0{
            let now = Instant::now();

            self.delay_timer -= 1;

            if let Some(dur) = Duration::from_secs_f64(PERIOD_DELAY_AS_SECS).checked_sub(now.elapsed()) {
                sleep(dur);
            }
        }
    }
}