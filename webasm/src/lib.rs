extern crate console_error_panic_hook;

use emulator::*;
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

const PIXEL_SIZE: usize = 15;
const CANVAS_WIDTH: u32 = (PIXEL_SIZE * DISPLAY_WIDTH) as u32;
const CANVAS_HEIGHT: u32 = (PIXEL_SIZE * DISPLAY_HEIGHT) as u32;
const TICKS_PER_FRAME: usize = 10;

#[wasm_bindgen]
struct EmulatorHandler {
    emulator: Emulator,
    ctx: CanvasRenderingContext2d,
}

#[wasm_bindgen]
impl EmulatorHandler {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: HtmlCanvasElement) -> Self {
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
            ctx
        }
    }

    #[wasm_bindgen]
    pub fn load_rom(&mut self, rom: &Uint8Array) {
        self.emulator.load_rom(&rom.to_vec())
    }
    
    #[wasm_bindgen]
    pub fn tick(&mut self) {
        self.emulator.tick();
    }

    #[wasm_bindgen]
    pub fn tick_timers(&mut self) -> bool {
        self.emulator.tick_delay_timer();
        self.emulator.tick_sound_timer()
    }

    #[wasm_bindgen]
    pub fn draw_to_canvas(&mut self) {
        let display = self.emulator.display;
        self.ctx.clear_rect(0f64, 0f64, CANVAS_WIDTH as f64, CANVAS_HEIGHT as f64);
        display
            .chunks(DISPLAY_WIDTH)
            .enumerate()
            .for_each(|(y, chunk)| {
                chunk.into_iter().enumerate().for_each(|(x, pixel)| {
                    let color = if *pixel { "white" } else { "black" };
                    let (x, y) = (x * PIXEL_SIZE, y * PIXEL_SIZE);
                    self.ctx.set_fill_style(&JsValue::from_str(color));
                    let pixel_size = PIXEL_SIZE as f64;
                    self.ctx.fill_rect(x as f64, y as f64, pixel_size, pixel_size);
                })
            });
    }
}
