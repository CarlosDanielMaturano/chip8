#![allow(dead_code)] // for avoid wasm function being tagged as unused

extern crate console_error_panic_hook;
mod web_audio;

use emulator::*;
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;
use web_sys::{
    CanvasRenderingContext2d, 
    HtmlCanvasElement,
    KeyboardEvent,
};

const PIXEL_SIZE: usize = 15;
const CANVAS_WIDTH: u32 = (PIXEL_SIZE * DISPLAY_WIDTH) as u32;
const CANVAS_HEIGHT: u32 = (PIXEL_SIZE * DISPLAY_HEIGHT) as u32;
const TICKS_PER_FRAME: usize = 15;

fn key_to_hex(key: KeyboardEvent) -> Option<u8> {
    match key.key().as_str() {
        "1" => Some(0x1),
        "2" => Some(0x2),
        "3" => Some(0x3),
        "4" => Some(0xC),
        "q" => Some(0x4),
        "w" => Some(0x5),
        "e" => Some(0x6),
        "r" => Some(0xD),
        "a" => Some(0x7),
        "s" => Some(0x8),
        "d" => Some(0x9),
        "f" => Some(0xE),
        "z" => Some(0xA),
        "x" => Some(0x0),
        "c" => Some(0xB),
        "v" => Some(0xF),
        _ => None,
    }
}

#[wasm_bindgen]
struct EmulatorHandler {
    emulator: Emulator,
    ctx: CanvasRenderingContext2d,
}

#[wasm_bindgen]
impl EmulatorHandler {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: HtmlCanvasElement) -> Self {
        // sets a panic hook for better error loggin
        console_error_panic_hook::set_once(); 
        canvas.set_width(CANVAS_WIDTH);
        canvas.set_height(CANVAS_HEIGHT);

        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        Self {
            emulator: Emulator::new(),
            ctx,
        }
    }

    #[wasm_bindgen]
    pub fn load_rom(&mut self, rom: &Uint8Array) {
        self.emulator.load_rom(&rom.to_vec())
    }

    #[wasm_bindgen]
    pub fn reset(&mut self) {
        self.emulator.reset();
    }
    
    #[wasm_bindgen]
    pub fn tick(&mut self) {
        self.emulator.tick();
    }

    #[wasm_bindgen]
    pub fn tick_timers(&mut self) {
        self.emulator.tick_delay_timer();
        if self.emulator.tick_sound_timer() {
            web_audio::play_sound();
        }
    }

    #[wasm_bindgen]
    pub fn draw_to_canvas(&mut self) {
        let display = self.emulator.display;

        self.ctx.set_fill_style(&JsValue::from_str("black"));
        self.ctx.fill_rect(0f64, 0f64, CANVAS_WIDTH as f64, CANVAS_HEIGHT as f64);

        display
            .chunks(DISPLAY_WIDTH)
            .enumerate()
            .for_each(|(y, chunk)| {
                chunk.into_iter().enumerate().for_each(|(x, pixel)| {
                    if *pixel {
                        let (x, y) = (x * PIXEL_SIZE, y * PIXEL_SIZE);
                        self.ctx.set_fill_style(&JsValue::from_str("white"));
                        let pixel_size = PIXEL_SIZE as f64;
                        self.ctx.fill_rect(x as f64, y as f64, pixel_size, pixel_size);
                    }
                })
            });
    }

    #[wasm_bindgen]
    pub fn handle_key_press(&mut self, key: KeyboardEvent, pressed: bool) {
        if let Some(hex) = key_to_hex(key) {
            self.emulator.set_key_press(hex, pressed)
        }
    }

}
