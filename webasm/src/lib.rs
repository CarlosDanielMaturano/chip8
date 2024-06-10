use emulator::*;
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

const PIXEL_SIZE: usize = 15;

#[wasm_bindgen]
struct EmulatorHandler {
    emulator: Emulator,
}

#[wasm_bindgen]
impl EmulatorHandler {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            emulator: Emulator::new(),
        }
    }

    #[wasm_bindgen]
    pub fn load_rom(&mut self, rom: &Uint8Array) {
        self.emulator.load_rom(&rom.to_vec())
    }

    #[wasm_bindgen]
    pub fn configure_canvas(&mut self, canvas: &HtmlCanvasElement) {
        let canvas_width = (PIXEL_SIZE * DISPLAY_WIDTH) as u32;
        let canvas_height = (PIXEL_SIZE * DISPLAY_HEIGHT) as u32;
        canvas.set_width(canvas_width);
        canvas.set_height(canvas_height);
    }

    #[wasm_bindgen]
    pub fn draw_to_canvas(&mut self, ctx: &CanvasRenderingContext2d) {
        let display = self.emulator.display;
        display
            .chunks(DISPLAY_WIDTH)
            .enumerate()
            .for_each(|(y, chunk)| {
                chunk.into_iter().enumerate().for_each(|(x, pixel)| {
                    if *pixel {
                        let (x, y) = (x * PIXEL_SIZE, y * PIXEL_SIZE);
                        ctx.set_fill_style(&JsValue::from_str("green"));
                        let pixel_size = PIXEL_SIZE as f64;
                        ctx.fill_rect(x as f64, y as f64, pixel_size, pixel_size);
                    }
                })
            });
    }
}
