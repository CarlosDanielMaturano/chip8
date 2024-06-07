use emulator::*;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;

const WINDOW_SCALE: u32 = 15;
const WINDOW_WIDTH: u32 = DISPLAY_WIDTH as u32 * WINDOW_SCALE;
const WINDOW_HEIGHT: u32 = DISPLAY_HEIGHT as u32 * WINDOW_SCALE;

const ONE_SECOND_AS_MILI: u32 = 10u32.pow(9);
const TICKS_PER_FRAME: usize = 8;

fn main() {
    let Some(rom_path) = std::env::args().skip(1).next() else {
        eprintln!("Missing rom file path.");
        return
    };
    
    let buf = std::fs::read(rom_path);
    let Ok(buf) = buf else {
        let error = buf.unwrap_err();
        eprintln!("Error reading the rom content: {error} ");
        return;
    };

    let mut chip8 = Emulator::new();
    chip8.load_rom(&buf);

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

    let mut event_pump = sdl_context
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
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main_game_loop,
                _ => (),
            }
        }
        // clear the background
        canvas.set_draw_color(background_color);
        canvas.clear();

        // tick  the emulator
        for _ in 0..TICKS_PER_FRAME {
            chip8.tick();
        }
        chip8.tick_timers();

        // draw the pixels
        canvas.set_draw_color(pixel_color);
        chip8.display.chunks(DISPLAY_WIDTH)
            .enumerate()
            .for_each(|(y, chunk)| {
                chunk
                    .into_iter()
                    .enumerate()
                    .for_each(|(x, pixel)| {
                        if *pixel {
                            let scale = WINDOW_SCALE as i32;
                            let (x_pos, y_pos) = (x as i32 * scale, y as i32* scale);
                            let scale = scale as u32;
                            let pixel_rect = Rect::new(x_pos, y_pos, scale, scale);
                            _ = canvas.fill_rect(pixel_rect);
                        }
                    })
            });

        canvas.present();
        std::thread::sleep(std::time::Duration::new(0, ONE_SECOND_AS_MILI / 60));
    }
}
