use std::env;
use chip::Chip8;
use rom::Rom;

pub mod stack;
pub mod chip;
pub mod rom;

fn main() {
    let mut args = env::args();

    let rom_path = match args.nth(1) {
        Some(r) => r,
        None => panic!("Insufficient arguments passed")
    };

    
    let rom = Rom::read_rom(&rom_path);

    let mut chip8 = Chip8::new(rom);

    chip8.run();
}