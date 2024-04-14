use std::env;
use chip::Chip8;
use rom::Rom;
use raylib::prelude::*;
use bit::BitIndex;

pub mod stack;
pub mod chip;
pub mod rom;

fn main() {
    let mut args = env::args();
    let (mut raylib_handler, mut raylib_thread_handler) = raylib::init()
        .size(640, 320)
        .build();

    let rom_path = match args.nth(1) {
        Some(r) => r,
        None => panic!("Insufficient arguments passed")
    };

    
    let rom = Rom::read_rom(&rom_path);

    let mut chip8 = Chip8::new(rom);

    chip8.run();
}

fn draw(raylib_handler: &mut RaylibHandle, raylib_thread_handler: &RaylibThread, chip8: &Chip8){
    let mut draw_handler = raylib_handler.begin_drawing(raylib_thread_handler);
    draw_handler.clear_background(Color::BLACK);

    for (y, row) in chip8.display.iter().enumerate(){
        for (x ,i) in (0..64).rev().enumerate(){
            if row.bit(i) {
                draw_handler.draw_rectangle(x as i32 * 10, y as i32 * 10, 10, 10, Color::WHITE);
            }
        }
    }
}