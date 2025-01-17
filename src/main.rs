use chip::Chip8;
use rom::Rom;
use raylib::prelude::*;
use bit::BitIndex;
use std::time::{Instant, Duration};
use spin_sleep::sleep;
use config::Config;

pub mod stack;
pub mod chip;
pub mod rom;
pub mod timer;
pub mod config;
pub mod gui;

static mut DELTA_TIME: f64= 0.0;

fn main() {
    let (play_flag, config): (bool, Config)= gui::run();

    if !play_flag { return }
    
    let rom = Rom::read_rom(&config.rom_path);
    let mut chip8 = Chip8::new(rom, &config);

    let (mut raylib_handler, raylib_thread_handler) = raylib::init()
    .size(640, 320)
    .build();

    let cycle = 1.0_f64 / config.cpu_hertz as f64;
    raylib_handler.set_target_fps(config.cpu_hertz);
    
    while !raylib_handler.window_should_close() {
        let now = Instant::now();
        let key_pressed = is_key_down(&mut raylib_handler);

        chip8.run_cycle(key_pressed);

        if chip8.draw_flag { 
            draw(&mut raylib_handler, &raylib_thread_handler, &chip8); 
            chip8.draw_flag = false;
        }

        if let Some(dur) = Duration::from_secs_f64(cycle).checked_sub(now.elapsed()){
            sleep(dur);
        }

        unsafe { DELTA_TIME = now.elapsed().as_secs_f64(); }
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

fn is_key_down(raylib_handler: &mut RaylibHandle) -> Option<u8> {
    if raylib_handler.is_key_down(KeyboardKey::KEY_ONE) { return Some(0) }
    if raylib_handler.is_key_down(KeyboardKey::KEY_TWO) { return Some(1) }
    if raylib_handler.is_key_down(KeyboardKey::KEY_THREE) { return Some(2) }
    if raylib_handler.is_key_down(KeyboardKey::KEY_FOUR) { return Some(3) }
    if raylib_handler.is_key_down(KeyboardKey::KEY_Q) { return Some(4) }
    if raylib_handler.is_key_down(KeyboardKey::KEY_W) { return Some(5) }
    if raylib_handler.is_key_down(KeyboardKey::KEY_E) { return Some(6) }
    if raylib_handler.is_key_down(KeyboardKey::KEY_R) { return Some(7) }
    if raylib_handler.is_key_down(KeyboardKey::KEY_A) { return Some(8) }
    if raylib_handler.is_key_down(KeyboardKey::KEY_S) { return Some(9) }
    if raylib_handler.is_key_down(KeyboardKey::KEY_D) { return Some(10) }
    if raylib_handler.is_key_down(KeyboardKey::KEY_F) { return Some(11) }
    if raylib_handler.is_key_down(KeyboardKey::KEY_Z) { return Some(12) }
    if raylib_handler.is_key_down(KeyboardKey::KEY_X) { return Some(13) }
    if raylib_handler.is_key_down(KeyboardKey::KEY_C) { return Some(14) }
    if raylib_handler.is_key_down(KeyboardKey::KEY_V) { return Some(15) }
    None
}