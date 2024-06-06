pub const DISPLAY_WIDTH: usize = 64; // 64 pixels
pub const DISPLAY_HEIGHT: usize = 32; // 32 pixels
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
    dt: u8,  // delay timer
    st: u8,  // sound timer
    pc: u16, // program counter
    sp: u8,  // stack pointer
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
            display: [false; DISPLAY_SIZE],
        }
    }

    // push the value into the stack
    fn push(&mut self, value: u16) {
        self.stack[self.sp as usize] = value;
        self.sp += 1;
    }

    // get the top value of the stack
    fn pop(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp as usize]
    }

    fn tick_timer(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }
        if self.st > 0 {
            if self.st == 1 {
                // here commes a beep
            }
            self.st -= 1;
        }
    }

    fn tick(&mut self) {
        let instruction = self.get_next_instruction();

        // decode the instruction
        let decoded_ins: [u8; 4] = [
            ((instruction >> 12) & 0xF000) as u8, // first 4 bits
            ((instruction >> 4)  & 0x0F00) as u8,
            ((instruction >> 8)  & 0x00F0) as u8,
            ((instruction >> 0)  & 0x000F) as u8, // last 4 bits
        ];

        match decoded_ins {
            _ => unreachable!("Error. Unknown instruction: 0x{:x}", instruction)
        }
    }

    fn get_next_instruction(&mut self) -> u16 {
        let pc = self.pc as usize;
        let higher_byte = self.ram[pc] as u16;
        let lower_byte = self.ram[pc + 1] as u16;
        let instruction: u16 = (higher_byte << 8) | lower_byte;
        self.pc += 2;
        instruction
    }
}
