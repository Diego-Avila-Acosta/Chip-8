use std::ffi::CString;
use rfd;
use std::env;
use crate::config::Config;
use raylib::prelude::*;
use raylib::ffi::GuiControl::*;
use raylib::ffi::GuiControlProperty::*;

const SCREEN_WIDTH: i32 = 690;
const SCREEN_HEIGHT: i32 = 200;

pub fn run() -> (bool, Config){
    let (mut raylib, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Chip-8 Emulator")
        .build();
    
    raylib.set_target_fps(60);

    let mut rom_path: String = String::new();
    let mut rom_offset: i32 = 512; // u16
    let mut cpu_hertz: i32 = 700; // u32
    let mut delay_timer_hertz: i32 = 60; // u8
    let mut sound_timer_hertz: i32 = 60; // u8

    let mut cpu_hertz_flag = false;
    let mut rom_offset_flag = false;
    let mut dt_hertz_flag = false;
    let mut st_hertz_flag = false;
    let mut play_flag = false;
    let mut rom_empty = false;

    while !raylib.window_should_close() && !play_flag{
        if raylib.is_file_dropped() {
            rom_path = raylib.get_dropped_files().pop().unwrap();
            raylib.clear_dropped_files();
            rom_empty = false;
        }

        let mut draw = raylib.begin_drawing(&thread);

        let hex = draw.gui_get_style(DEFAULT, BASE_COLOR_NORMAL as i32);
        draw.clear_background(Color::get_color(hex));

        // Rom
        let image_text = draw.gui_icon_text(guiIconName::RICON_FILETYPE_PLAY, Some(&CString::new("ROM").unwrap()));
        draw.gui_label(rrect(10, 10, 100, 20), Some(&CString::new(image_text.as_str()).unwrap()));
        draw.gui_label(rrect(170, 10, 100, 20), Some(&CString::new(rom_path.as_str()).unwrap()));
        let browse_clicked = draw.gui_button(rrect( 90, 10, 70, 20), Some(&CString::new("Browse File").unwrap()));

        if browse_clicked {
            let option_file = rfd::FileDialog::new()
                .add_filter("rom", &["ch8"])
                .set_directory(&env::current_dir().unwrap())
                .pick_file();

            match option_file {
                Some(path_buff) => {
                    rom_empty = false;
                    rom_path = String::from(path_buff.to_str().unwrap());
                }
                None => ()
            }
        }

        // Values box

        draw.gui_label(rrect(170, 40, 100, 20), Some(&CString::new("Address offset of the ROM in memory (default: 512)").unwrap()));
        draw_value_box(&mut draw, 40 , &mut rom_offset, &mut rom_offset_flag, "ROM OFFSET", 0, u16::MAX as i32);

        draw.gui_label(rrect(170, 70, 100, 20), Some(&CString::new("How many instruction per second the CPU will execute (normal values 500-1000)").unwrap()));
        draw_value_box(&mut draw, 70 , &mut cpu_hertz, &mut cpu_hertz_flag, "CPU HERTZ", 0, i32::MAX);

        draw.gui_label(rrect(170, 100, 100, 20), Some(&CString::new("Count down per second. Used for timing events in games").unwrap()));
        draw_value_box(&mut draw, 100 , &mut delay_timer_hertz, &mut dt_hertz_flag, "DT HERTZ", 0, u8::MAX as i32);

        draw.gui_label(rrect(170, 130, 100, 20), Some(&CString::new("Count down per second. Used for sound effects").unwrap()));
        draw_value_box(&mut draw, 130 , &mut sound_timer_hertz, &mut st_hertz_flag, "ST HERTZ", 0, u8::MAX as i32);

        // Play Button

        if draw.gui_button(rrect(10, 160, 70, 20), Some(&CString::new("Play").unwrap())){
            if rom_path != "" { play_flag = true; }
            else {
                rom_empty = true;
            }
        }

        if rom_empty {
            draw.gui_label(rrect(90, 160, 100, 20), Some(&CString::new("Please select a ROM").unwrap()));
        }

        // GitHub

        let image = draw.gui_icon_text(guiIconName::RICON_HEART, None);
        draw.gui_label(rrect(620, 160, 100, 20), Some(&CString::new("Github").unwrap()));
        if draw.gui_button(rrect(660, 160, 20, 20), Some(&CString::new(image.as_str()).unwrap())) {
            open_url("https://github.com/Diego-Avila-Acosta");
        }

    }

    (play_flag, Config {
        rom_path,
        rom_offset: rom_offset as u16,
        cpu_hertz: cpu_hertz as u32,
        delay_timer_hertz: delay_timer_hertz as u8,
        sound_timer_hertz: sound_timer_hertz as u8
    })
}

fn draw_value_box(draw: &mut RaylibDrawHandle, y: i32, value: &mut i32, flag: &mut bool, text: &str, min: i32, max: i32){
    draw.gui_label(rrect(10, y, 100, 20), Some(&CString::new(text).unwrap()));

    if draw.gui_value_box(
        rrect(90, y, 70,20),
        None,
        value,
        min,
        max,
        *flag,
    ) { *flag = !(*flag); }
}