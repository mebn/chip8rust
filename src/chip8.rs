use crate::screen;

pub struct Chip8 {
    memory: [u16; 4096],
    v: [u8; 16], // registers
    i: u16, // used to store memory addresses, lowest (rightmost) 12 bits
    pc: u16, // program counter, should start at 0x200
    sp: u8, // stack pointer
    stack: [u16; 16],
    delay_timer: u16,
    sound_timer: u16,
    screen: screen::Screen
}

impl Chip8 {
    pub fn new(screen: screen::Screen) -> Self {
        Self {
            memory: [0; 4096],
            v: [0; 16],
            i: 0,
            pc: 0x200,
            sp: 0,
            stack: [0; 16],
            delay_timer: 0,
            sound_timer: 0,
            screen
        }
    }
    fn init_font(&mut self) {
        // use memory position 0x000 to 0x1FF to store font
        let sprites = [
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
            0xF0, 0x80, 0xF0, 0x80, 0x80  // F
        ];

        for i in 0..sprites.len() {
            self.memory[i] = sprites[i];
        }
    }

    fn handle_instruction(&mut self, instr: u16) {
        self.pc += 2; // each instruction is 2 bytes long

        let x = (instr & 0x0F00) >> 8;
        let y = (instr & 0x00F0) >> 4;

        match instr & 0xF000 {
            0x0 => match instr {
                // 00E0 - CLS
                0x00E0 => self.screen.clear(),
                // 00EE - RET
                0x0EE => {
                    self.pc = self.stack[self.sp as usize];
                    self.sp -= 1;
                },
                _ => {}
            },
            // 1nnn - JP addr
            0x1 => self.pc = instr & 0x0FFF,
            // 2nnn - CALL addr
            0x2 => {
                self.sp += 1;
                self.stack[self.sp as usize] = self.pc;
                self.pc = instr & 0x0FFF;
            },
            // 3xkk - SE Vx, byte
            0x3 => if self.v[x as usize] as u16 == (instr & 0x00FF) {
                self.pc += 2;
            },
            // 4xkk - SNE Vx, byte
            0x4 => if self.v[self.sp as usize] as u16 != (instr & 0x00FF) {
                self.pc += 2;
            },
            // 5xy0 - SE Vx, Vy
            0x5 => if x == y {
                self.pc += 2;
            },
            // 6xkk - LD Vx, byte
            0x6 => self.v[x as usize] = (instr & 0xFF) as u8,
            // 7xkk - ADD Vx, byte
            0x7 => self.v[x as usize] += (instr & 0xFF) as u8,
            0x8 => match instr & 0xF {
                // 8xy0 - LD Vx, Vy
                0x0 => self.v[x as usize] = self.v[y as usize],
                // 8xy1 - OR Vx, Vy
                0x1 => self.v[x as usize] |= self.v[y as usize],
                // 8xy2 - AND Vx, Vy
                0x2 => self.v[x as usize] &= self.v[y as usize],
                // 8xy3 - XOR Vx, Vy
                0x3 => self.v[x as usize] ^= self.v[y as usize],
                // 8xy4 - ADD Vx, Vy
                0x4 => {
                    let sum = self.v[x as usize] as u16 + self.v[y as usize] as u16;
                    self.v[x as usize] = (sum & 0x00FF) as u8;
                    self.v[0xF] = 0;
                    if sum > 0xFF {
                        self.v[0xF] = 1;
                    }
                },
                // 8xy5 - SUB Vx, Vy
                0x5 => {
                    self.v[0xF] = 0;
                    if self.v[x as usize] > self.v[y as usize] {
                        self.v[0xF] = 1;
                    }
                    self.v[x as usize] -= self.v[y as usize];
                },
                // 8xy6 - SHR Vx {, Vy}
                0x6 => {
                    self.v[0xF] = self.v[x as usize] & 0x1;
                    self.v[x as usize] >>= 1;
                },
                // 8xy7 - SUBN Vx, Vy
                0x7 => {
                    self.v[0xF] = 0;
                    if self.v[y as usize] > self.v[x as usize] {
                        self.v[0xF] = 1;
                    }
                    self.v[x as usize] = self.v[y as usize] - self.v[x as usize];
                },
                // 8xyE - SHL Vx {, Vy}
                0xE => {
                    self.v[0xF] = self.v[x as usize] & 0x80;
                    self.v[x as usize] <<= 1;
                 },
                _ => {}
            },
            // 9xy0 - SNE Vx, Vy
            0x9 => if self.v[x as usize] != self.v[y as usize] {
                self.pc += 2;
            },
            // Annn - LD I, addr
            0xA => self.i = instr & 0xFFF,
            // Bnnn - JP V0, addr
            0xB => self.pc = (instr & 0xFFF) + self.v[0] as u16,
            // Cxkk - RND Vx, byte
            0xC => {
                use rand::Rng;
                let rand = rand::thread_rng().gen_range(0..=255);
                self.v[x as usize] = rand & (instr & 0xFF) as u8;
            },
            _ => {}
        }
    }
}
