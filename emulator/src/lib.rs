pub const DISPLAY_WIDTH: usize = 64; // 64 pixels
pub const DISPLAY_HEIGHT: usize = 32; // 32 pixels
const DISPLAY_SIZE: usize = DISPLAY_WIDTH * DISPLAY_HEIGHT; // 2048 pixels

const RAM_SIZE: usize = 4096;
const REGISTER_SIZE: usize = 16;
const STACK_SIZE: usize = 16;
const KEYS_SIZE: usize = 16;
const RAM_START_ADDR: usize = 0x200;

const FONTSET_SIZE: usize = 80;
const FONTSET: [u8; FONTSET_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];


pub struct Emulator {
    ram: [u8; RAM_SIZE],
    v_reg: [u8; REGISTER_SIZE],
    i_reg: u16,
    dt: u8,  // delay timer
    st: u8,  // sound timer
    pc: u16, // program counter
    sp: u8,  // stack pointer
    stack: [u16; STACK_SIZE],
    keys: [bool; KEYS_SIZE], // array for storing keyboard input
    // array for storing pixels stater
    // because a pixel is either on or off, using bool is fine
    pub display: [bool; DISPLAY_SIZE],
}

impl Emulator {
    pub fn new() -> Self {
        let mut emu = Self {
            ram: [0; RAM_SIZE],
            v_reg: [0; REGISTER_SIZE],
            i_reg: 0,
            dt: 0,
            st: 0,
            sp: 0,
            pc: RAM_START_ADDR as u16,
            stack: [0; STACK_SIZE],
            keys: [false; KEYS_SIZE],
            display: [false; DISPLAY_SIZE],
        };
        // load the font into the ram
        emu.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
        emu
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        let start = RAM_START_ADDR;
        let end = RAM_START_ADDR + rom.len();
        self.ram[start..end].copy_from_slice(rom);
    }

    pub fn set_key_press(&mut self, code: u8, pressed: bool) {
        self.keys[code as usize] = pressed
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

    pub fn tick_timers(&mut self) {
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

    pub fn tick(&mut self) {
        let instruction = self.get_next_instruction();

        // decode the instruction
        let decoded_instruction: [u16; 4] = [
            ((instruction & 0xF000) >> 12), // first 4 bits
            ((instruction & 0x0F00) >> 8) ,
            ((instruction & 0x00F0) >> 4) ,
            ((instruction & 0x000F) >> 0) , // last 4 bits
        ];

        match decoded_instruction {
            // NOOP
            [0, 0, 0, 0] => (),
            // CLS -> clear the display
            [0, 0, 0xE, 0] => {
                self.display = [false; DISPLAY_SIZE];
            }
            // RET -> return from a subroutine
            [0, 0, 0xE, 0xE] => {
                self.pc = self.pop();
            }
            // JMP ADDR -> jump to nnn
            [1, ..] => {
                let nnn = instruction & 0x0FFF;
                self.pc = nnn;
            }
            // CALL ADDR -> call subroutine at nnn
            [2, ..] => {
                let nnn = instruction & 0x0FFF;
                self.push(self.pc);
                self.pc = nnn;
            }
            // SE Vx, byte -> Skip next instruction if Vx == kk
            [3, x, ..] => {
                let kk = (instruction & 0x00FF) as u8;
                if self.v_reg[x as usize] == kk {
                    self.pc += 2;
                }
            }
            // SNE Vx, byte -> Skip next instruction if Vx != kk
            [4, x, ..] => {
                let kk = (instruction & 0xFF) as u8;
                if self.v_reg[x as usize] != kk {
                    self.pc += 2;
                }
            }
            // SE, Vx, Vy -> Skip next instruction if Vx == Vy
            [5, x, y, 0] => {
                if self.v_reg[x as usize] == self.v_reg[y as usize] {
                    self.pc += 2;
                }
            }
            // LD Vx, byte -> Set Vx == kk
            [6, x, ..] => {
                let kk = (instruction & 0xFF) as u8;
                self.v_reg[x as usize] = kk;
            }
            // ADD Vx, byte -> Set Vx = Vx + kk
            [7, x, ..] => {
                let x = x as usize;
                let kk = (instruction & 0xFF) as u8;
                self.v_reg[x] = self.v_reg[x].wrapping_add(kk);
            }
            // LD Vx, Vy -> Set Vx = Vy
            [8, x, y, 0] => {
                self.v_reg[x as usize] = self.v_reg[y as usize];
            }
            // OR Vx, Vy -> Set Vx = Vx OR Vy
            [8, x, y, 1] => {
                self.v_reg[x as usize] |= self.v_reg[y as usize];
            }
            // AND Vx, Vy -> Set Vx = Vx AND Vy
            [8, x, y, 2] => {
                self.v_reg[x as usize] &= self.v_reg[y as usize];
            }
            // XOR Vx, Vy -> Set Vx = Vx XOR Vy
            [8, x, y, 3] => {
                self.v_reg[x as usize] ^= self.v_reg[y as usize];
            }
            // ADD Vx, Vy -> Set Vx = Vx + Vy, Set VF = carry
            [8, x, y, 4] => {
                let (x, y) = (x as usize, y as usize);
                let (vx, carry) = self.v_reg[x].overflowing_add(self.v_reg[y]);
                self.v_reg[x] = vx;
                self.v_reg[0xF] = carry as u8;
            }
            // SUB Vx, Vy -> SET Vx = Vx - Vy, SET VF = NOT borrow
            [8, x, y, 5] => {
                let (x, y) = (x as usize, y as usize);
                let (vx, borrow) = self.v_reg[x].overflowing_sub(self.v_reg[y]);
                self.v_reg[x] = vx;
                self.v_reg[0xF] = (!borrow) as u8;
            }
            // SHR, Vx {, Vy} -> Set Vx = Vx SHR 1;
            [8, x, _, 6] => {
                let x = x as usize;
                let lsb = self.v_reg[x] & 1; // least significant bit of Vx
                self.v_reg[x] >>= 1; // divide Vx by 2 (right bit shift)
                self.v_reg[0xF] = lsb;
            }
            // SUBN, Vx, Vy -> Set Vx = Vy - Vx, set VF = NOT borrow
            [8, x, y, 7] => {
                let (x, y) = (x as usize, y as usize);
                let (vx, borrow) = self.v_reg[y].overflowing_sub(self.v_reg[x]);
                self.v_reg[x] = vx;
                self.v_reg[0xF] = (!borrow) as u8;
            }
            // SHL, Vx {, Vy} -> Set Vx = Vx SHL 1
            [8, x, _, 0xE] => {
                let x = x as usize;
                let msb = (self.v_reg[x] >> 7) & 1; // most significant bit of Vx
                self.v_reg[x] <<= 1; // multiply Vx by 2 (left bit shift)
                self.v_reg[0xF] = msb;
            }
            // SNE, Vx, Vy -> Skip next instruction if Vx != Vy
            [9, x, y, 0] => {
                if self.v_reg[x as usize] != self.v_reg[y as usize] {
                    self.pc += 2;
                }
            }
            // LD I, addr -> Set I Register = nnn;
            [0xA, ..] => {
                let nnn = instruction & 0xFFF;
                self.i_reg = nnn;
            }
            // JP, V0, addr -> Jump to location nnn + V0
            [0xB, ..] => {
                let nnn = instruction & 0xFFF;
                self.pc = nnn + self.v_reg[0] as u16;
            }
            // RND Vx, byte -> Set Vx = random_byte AND kk;
            [0xC, x, ..] => {
                let kk = (instruction & 0xFF) as u8;
                self.v_reg[x as usize] = rand::random::<u8>() & kk;
            }
            // DRW Vx, Vy, nibble -> Display the n-byte sprite starting
            // at memory location I at (Vx, Vy), SET VF = collision
            [0xD, x, y, n] => {
                let (x, y) = (x as usize, y as usize);
                let vx = self.v_reg[x];
                let vy = self.v_reg[y];
                let i = self.i_reg as usize;

                self.v_reg[0xF] = 0;

                for y in 0..n {
                    let pixel = self.ram[i + y as usize];
                    for x in 0..8 {
                        let msb = 0x80; // most significant bit of the pixel
                        if (pixel & (msb >> x)) != 0 {
                            let (d_width, d_height) = (DISPLAY_WIDTH as u16, DISPLAY_HEIGHT as u16);
                            let wrapped_x = (vx as u16 + x) % d_width;
                            let wrapped_y = (vy as u16 + y) % d_height;
                            // index of the pixel
                            let idx = (wrapped_x + wrapped_y * d_width) as usize;

                            self.display[idx] ^= true;

                            if !self.display[idx] {
                                self.v_reg[0xF] = 1;
                            }
                        }
                    }
                }
            }
            // SKP Vx -> Skip next instruction if key with value of Vx is pressed
            [0xE, x, 9, 0xE] => {
                if self.keys[self.v_reg[x as usize] as usize] {
                    self.pc += 2;
                }
            }
            // SKP Vx -> Skip next instruction if key with value of Vx is not pressed
            [0xE, x, 0xA, 1] => {
                if !self.keys[self.v_reg[x as usize] as usize] {
                    self.pc += 2;
                }
            }
            // LD Vx, DT -> Set Vx = delay timer value
            [0xF, x, 0, 7] => {
                self.v_reg[x as usize] = self.dt;
            }
            // LD, Vx, K -> Wait for a key press, store the value of the key in Vx
            [0xF, x, 0, 0xA] => {
                let mut pressed = false;
                for (idx, key) in self.keys.into_iter().enumerate() {
                    if key {
                        self.v_reg[x as usize] = idx as u8;
                        pressed = true;
                        break;
                    }
                }

                // Continue wating for the key press
                if !pressed {
                    self.pc -= 2;
                }
            }
            // LD DT, Vx -> Set delay timer = Vx
            [0xF, x, 1, 5] => {
                self.dt = self.v_reg[x as usize];
            }
            // LD ST, Vx -> Set sound timer = Vx
            [0xF, x, 1, 8] => {
                self.st = self.v_reg[x as usize];
            }
            // ADD I, Vx -> Set i_reg = i_reg + Vx
            [0xF, x, 1, 0xE] => {
                self.i_reg += self.v_reg[x as usize] as u16;
            }
            // LD F, Vx -> Set i_reg = location of sprite for digit Vx
            [0xF, x, 2, 9] => {
                self.i_reg = (self.v_reg[x as usize] * 0x5) as u16;
            }
            // LD B, Vx -> Store BCD representation of Vx 
            // in memory locations  I, I + 1, and I + 2
            [0xF, x, 3, 3] => {
                let i = self.i_reg as usize;
                let x = x as usize;
                self.ram[i]  = self.v_reg[x] / 100;
                self.ram[i + 1]  = (self.v_reg[x] / 10) % 10;
                self.ram[i + 2]  = self.v_reg[x] % 10; 
            }
            // LD [I], Vx -> Store registers V0 through VX
            // in memory starting at the location  I
            [0xF, x, 5, 5] => {
                let x = x as usize;
                let i = self.i_reg as usize;
                for idx in 0..=x {
                    self.ram[i + idx] = self.v_reg[idx];
                }
            }
            // LD, Vx, [I] -> Read registers V0 through
            // VX from memory starting at location I 
            [0xF, x, 6, 5] => {
                let x = x as usize;
                let i = self.i_reg as usize;
                for idx in 0..=x {
                    self.v_reg[idx] = self.ram[i + idx];
                }
            }
            _ => { 
                dbg!(decoded_instruction);
                unreachable!("Error. Unknown instruction: 0x{:x}", instruction) 
            },
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
