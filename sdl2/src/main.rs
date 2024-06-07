use emulator::*;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

const WINDOW_SCALE: u32 = 15;
const WINDOW_WIDTH: u32 = DISPLAY_WIDTH as u32 * WINDOW_SCALE;
const WINDOW_HEIGHT: u32 = DISPLAY_HEIGHT as u32 * WINDOW_SCALE;

const ONE_SECOND_AS_MILI: u32 = 10u32.pow(9);

fn main() {
    let sdl_context = sdl2::init().expect("Failed to initialize sdl2 context");
    let video_subsystem = sdl_context
        .video()
        .expect("Failed to initialize the sdl2 video_subsystem");

    let window = video_subsystem
        .window("Chip-8 Emulator", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .expect("Failed to create a sdl2 window");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Failed to create a cavas from the sdl2 window");

    
    let  mut event_pump = sdl_context
        .event_pump()
        .expect("Failed to initialize the event pump");

    let background_color = sdl2::pixels::Color::BLACK;
    let pixel_color = sdl2::pixels::Color::WHITE;

    canvas.set_draw_color(background_color);
    canvas.clear();
    canvas.present();

    'main_game_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'main_game_loop
                }
                _ => ()
            }
        }
        // clear the background
        canvas.set_draw_color(background_color);
        canvas.clear();

        canvas.present();
        std::thread::sleep(std::time::Duration::new(0, ONE_SECOND_AS_MILI / 60));
    }
}
