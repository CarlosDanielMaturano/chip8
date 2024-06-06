pub const DISPLAY_WIDTH: usize = 64;  // 64 pixels
pub const DISPLAY_HEIGHT: usize = 32;  // 32 pixels
const DISPLAY_SIZE: usize = DISPLAY_WIDTH * DISPLAY_HEIGHT; // 2048 pixels

const RAM_SIZE: usize = 4096;
const REGISTER_SIZE: usize = 16;
const STACK_SIZE: usize = 16;
const KEYS_SIZE: usize = 16;
const RAM_START_ADDR: u16 = 0x200;

pub struct Emulator {
    ram: [u8; RAM_SIZE],
    v_reg: [u8; REGISTER_SIZE],
    i_reg: u16,
    dt: u8, // delay timer
    st: u8, // sound timer
    pc: u16, // program counter
    sp: u8, // stack pointer
    stack: [u16; STACK_SIZE],
    keys: [u8; KEYS_SIZE], // array for storing keyboard input
    // array for storing pixels stater 
    // because a pixel is either on or off, using bool is fine
    display: [bool; DISPLAY_SIZE], 
}

impl Emulator {
    pub fn new() -> Self {
        Self {
            ram: [0; RAM_SIZE],
            v_reg: [0; REGISTER_SIZE],
            i_reg: 0,
            dt: 0,
            st: 0,
            sp: 0,
            pc: RAM_START_ADDR,
            stack: [0; STACK_SIZE],
            keys: [0; KEYS_SIZE],
            display: [false; DISPLAY_SIZE]
        }
    }
}
