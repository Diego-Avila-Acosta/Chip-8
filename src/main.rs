use std::env;
use chip::Chip8;
use rom::Rom;
use raylib::prelude::*;
use bit::BitIndex;
use std::time::{Instant, Duration};
use spin_sleep::sleep;

pub mod stack;
pub mod chip;
pub mod rom;
pub mod timer;

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

    let hertz: f64 = match args.next() {
        Some(r) => r.parse().expect("Hertz must be a number"),
        None => 700.0_f64
    };
    let cycle = 1.0_f64 / hertz;

    let mut chip8 = Chip8::new(rom);
    let mut delta_time: f64 = 0.0;

    while !raylib_handler.window_should_close() {
        let now = Instant::now();
        let key_pressed = match raylib_handler.get_key_pressed() {
            Some(key) => get_mapped_key(key),
            None => None
        };

        chip8.run_instruction(delta_time, key_pressed);

        draw(&mut raylib_handler, &raylib_thread_handler, &chip8);

        if let Some(dur) = Duration::from_secs_f64(cycle).checked_sub(now.elapsed()){
            sleep(dur);
        }

        delta_time = now.elapsed().as_secs_f64();
    }
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

fn get_mapped_key(key: KeyboardKey) -> Option<u8> {
    Some(match key{
        KeyboardKey::KEY_ONE => 0,
        KeyboardKey::KEY_TWO => 1,
        KeyboardKey::KEY_THREE => 2,
        KeyboardKey::KEY_FOUR => 3,
        KeyboardKey::KEY_Q => 4,
        KeyboardKey::KEY_W => 5,
        KeyboardKey::KEY_E => 6,
        KeyboardKey::KEY_R => 7,
        KeyboardKey::KEY_A => 8,
        KeyboardKey::KEY_S => 9,
        KeyboardKey::KEY_D => 10,
        KeyboardKey::KEY_F => 11,
        KeyboardKey::KEY_Z => 12,
        KeyboardKey::KEY_X => 13,
        KeyboardKey::KEY_C => 14,
        KeyboardKey::KEY_V => 15,
        _ => { return None }
    })
}