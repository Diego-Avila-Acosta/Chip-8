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

enum ArithmeticLogic {
    BitwiseOr,
    BitwiseAnd,
    BitwiseXor,
    AddsWithCarry,
    SubtractWithBorrow,
    SubtractYWithBorrow,
    ShiftLeft,
    ShiftRight,
}

enum Display{
    Clear,
    DisplayBytes(usize, usize, u8)
}

enum Subroutine {
    Return,
    Call(usize)
}

enum Skip {
    SkipIfEqual,
    SkipIfNotEqual,
    SkipIfKeyPressed,
    SkipIfKeyNotPressed
}

enum Register {
    Set,
    Add,
    SetToDelayTimer,
    StoreInMemory,
    ReadFromMemory
}

enum TimerInstruction{
    SetDelay,
    SetSound
}

enum IRegister {
    Set(u16),
    AddRegister(usize),
    SetToLocationSprite(usize)
}

enum Instruction {
    Display(Display),
    Subroutine(Subroutine),
    Skip(Skip, usize, u8),
    ArithmeticLogic(ArithmeticLogic, usize, u8),
    Jump(usize),
    Register(Register, usize, u8),
    IRegister(IRegister),
    Timer(TimerInstruction, usize),
    RandomByte(usize, u8),
    StoreBCD(usize),
    WaitKeyPress(usize)
}

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

    pub fn run_instruction(&mut self, delta_time: f64, key_pressed: Option<u8>) {
        self.delay_timer.check(delta_time);

        let instruction = self.fetch();
        let instruction_type = self.decode(instruction);
    }

    fn fetch(&mut self) -> u16{
        let instruction = ((self.memory[self.pc] as u16) << 8) + self.memory[self.pc + 1] as u16;
        self.pc += 2;
    
        instruction
    }

    fn decode(&self, instruction: u16) -> Instruction {
        let i = (0xF000 & instruction) << 12;
        let x = ((0x0F00 & instruction) << 8) as usize;
        let y = ((0x00F0 & instruction) << 4) as usize;
        let n = (0x000F & instruction) as u8;
        let nn = (0x00FF & instruction) as u8;
        let nnn = (0x0FFF & instruction) as u16;
    
        match i {
            0x0 => {
                match nn {
                    0xE0 => Instruction::Display(Display::Clear),
                    0xEE => Instruction::Subroutine(Subroutine::Return),
                    _ => todo!()
                }
            },
            0x1 => Instruction::Jump(nnn as usize),
            0x2 => Instruction::Subroutine(Subroutine::Call(nnn as usize)),
            0x3 => Instruction::Skip(Skip::SkipIfEqual, x, nn),
            0x4 => Instruction::Skip(Skip::SkipIfNotEqual, x, nn),
            0x5 => {
                let nn = self.registers[y];
                Instruction::Skip(Skip::SkipIfEqual, x, nn)
            },
            0x6 => Instruction::Register(Register::Set, x, nn),
            0x7 => Instruction::Register(Register::Add, x, nn),
            0x8 => {
                let nn = self.registers[y];
                match n {
                    0x0 => Instruction::Register(Register::Set, x, nn),
                    0x1 => Instruction::ArithmeticLogic(ArithmeticLogic::BitwiseOr, x, nn),
                    0x2 => Instruction::ArithmeticLogic(ArithmeticLogic::BitwiseAnd, x, nn),
                    0x3 => Instruction::ArithmeticLogic(ArithmeticLogic::BitwiseXor, x, nn),
                    0x4 => Instruction::ArithmeticLogic(ArithmeticLogic::AddsWithCarry, x, nn),
                    0x5 => Instruction::ArithmeticLogic(ArithmeticLogic::SubtractWithBorrow, x, nn),
                    0x6 => Instruction::ArithmeticLogic(ArithmeticLogic::ShiftRight, x, nn),
                    0x7 => Instruction::ArithmeticLogic(ArithmeticLogic::SubtractYWithBorrow, x, nn),
                    0xE => Instruction::ArithmeticLogic(ArithmeticLogic::ShiftLeft, x, nn),
                    _ => todo!()
                }
            },
            0x9 => {
                let nn = self.registers[y];
                Instruction::Skip(Skip::SkipIfNotEqual, x, nn)
            },
            0xA => Instruction::IRegister(IRegister::Set(nnn)),
            0xB => {
                let nnn = nnn + self.registers[0] as u16;
                Instruction::Jump(nnn as usize)
            },
            0xC => Instruction::RandomByte(x, nn),
            0xE => {
                match nn {
                    0x9E => Instruction::Skip(Skip::SkipIfKeyPressed, x, 0),
                    0xA1 => Instruction::Skip(Skip::SkipIfKeyNotPressed, x, 0),
                    _ => todo!()
                }
            },
            0xF => {
                match nn {
                    0x07 => Instruction::Register(Register::SetToDelayTimer, x, 0),
                    0x0A => Instruction::WaitKeyPress(x),
                    0x15 => Instruction::Timer(TimerInstruction::SetDelay, x),
                    0x18 => Instruction::Timer(TimerInstruction::SetSound, x),
                    0x1E => Instruction::IRegister(IRegister::AddRegister(x)),
                    0x29 => Instruction::IRegister(IRegister::SetToLocationSprite(x)),
                    0x33 => Instruction::StoreBCD(x),
                    0x55 => Instruction::Register(Register::StoreInMemory, x, 0),
                    0x65 => Instruction::Register(Register::ReadFromMemory, x, 0),
                    _ => todo!()
                }
            },
            _ => todo!()
        }
    }
}