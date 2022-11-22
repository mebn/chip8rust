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
    // screen: screen::Screen
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            memory: [0; 4096],
            v: [0; 16],
            i: 0,
            pc: 0x200,
            sp: 0,
            stack: [0; 16],
            delay_timer: 0,
            sound_timer: 0,
            // screen: screen::Screen {  }
        }
    }
    pub fn init_font(&mut self) {
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

    pub fn handle_instruction(&mut self, instr: u16) {
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
                }
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
                // 0x4 => 
            },
            _ => {}
        }
    }
}
