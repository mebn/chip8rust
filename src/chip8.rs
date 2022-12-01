use crate::{screen, controls};

pub struct Chip8 {
    memory: [u16; 4096],
    v: [u8; 16], // registers
    i: u16, // used to store memory addresses, lowest (rightmost) 12 bits
    pc: u16, // program counter, should start at 0x200
    sp: u8, // stack pointer
    stack: [u16; 16],
    delay_timer: u8,
    sound_timer: u8,
    is_paused: bool,
    screen: screen::Screen,
    controls: controls::Controls,
    batch_size: u32
}

impl Chip8 {
    pub fn new(screen: screen::Screen, controls: controls::Controls) -> Self {
        let mut chip8 = Chip8 {
            memory: [0; 4096],
            v: [0; 16],
            i: 0,
            pc: 0x200,
            sp: 0,
            stack: [0; 16],
            delay_timer: 0,
            sound_timer: 0,
            is_paused: false,
            screen,
            controls,
            batch_size: 10
        };

        chip8.init_font();

        chip8
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        // chip-8 programs stored in 0x200 - 0xFFF
        for (i, &instr) in rom.iter().enumerate() {
            self.memory[0x200 + i] = instr as u16;
        }
    }

    pub fn cycle(&mut self) {
        if self.is_paused { return; };

        let instr = self.memory[self.pc as usize] << 8 | self.memory[self.pc as usize + 1];
        self.handle_instruction(instr);

        self.update_timers();
        self.screen.draw();
    }

    pub fn system_loop(&mut self, fps: u32) {
        loop {
            if self.is_paused { return; };

            let instr = self.memory[self.pc as usize] << 8 | self.memory[self.pc as usize + 1];
            self.handle_instruction(instr);

            self.update_timers();
            self.screen.draw();

            let ms = (1000.0 / fps as f64) as u64;
            std::thread::sleep(std::time::Duration::from_millis(ms));
        }
    }

    fn update_timers(&mut self) {
        let decr = |timer: &mut u8| if *timer > 0 { *timer -= 1 };
        decr(&mut self.sound_timer);
        decr(&mut self.delay_timer);
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
        self.pc += 2; // each instruction is 2 bytes (16 bits) long

        let x = ((instr & 0x0F00) >> 8) as usize;
        let y = ((instr & 0x00F0) >> 4) as usize;

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
            0x3 => if self.v[x] as u16 == (instr & 0x00FF) {
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
            0x6 => self.v[x] = (instr & 0xFF) as u8,
            // 7xkk - ADD Vx, byte
            0x7 => self.v[x] += (instr & 0xFF) as u8,
            0x8 => match instr & 0xF {
                // 8xy0 - LD Vx, Vy
                0x0 => self.v[x] = self.v[y],
                // 8xy1 - OR Vx, Vy
                0x1 => self.v[x] |= self.v[y],
                // 8xy2 - AND Vx, Vy
                0x2 => self.v[x] &= self.v[y],
                // 8xy3 - XOR Vx, Vy
                0x3 => self.v[x] ^= self.v[y],
                // 8xy4 - ADD Vx, Vy
                0x4 => {
                    let sum = self.v[x] as u16 + self.v[y] as u16;
                    self.v[x] = (sum & 0x00FF) as u8;
                    self.v[0xF] = 0;
                    if sum > 0xFF {
                        self.v[0xF] = 1;
                    }
                },
                // 8xy5 - SUB Vx, Vy
                0x5 => {
                    self.v[0xF] = 0;
                    if self.v[x] > self.v[y] {
                        self.v[0xF] = 1;
                    }
                    self.v[x] -= self.v[y];
                },
                // 8xy6 - SHR Vx {, Vy}
                0x6 => {
                    self.v[0xF] = self.v[x] & 0x1;
                    self.v[x] >>= 1;
                },
                // 8xy7 - SUBN Vx, Vy
                0x7 => {
                    self.v[0xF] = 0;
                    if self.v[y] > self.v[x] {
                        self.v[0xF] = 1;
                    }
                    self.v[x] = self.v[y] - self.v[x];
                },
                // 8xyE - SHL Vx {, Vy}
                0xE => {
                    self.v[0xF] = self.v[x] & 0x80;
                    self.v[x] <<= 1;
                },
                _ => {}
            },
            // 9xy0 - SNE Vx, Vy
            0x9 => if self.v[x] != self.v[y] {
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
                self.v[x] = rand & (instr & 0xFF) as u8;
            },
            // Dxyn - DRW Vx, Vy, nibble
            0xD => {
                self.v[0xF] = 0;

                for row in 0..(instr & 0xF) {
                    for col in 0..8 {
                        if self.screen.draw_pixel(self.v[x] as u16 + col, self.v[y] as u16 + row) {
                            self.v[0xF] = 1;
                        }
                    }
                }
            },
            0xE => match instr & 0xFF {
                // Ex9E - SKP Vx
                0x9E => if self.controls.is_key_pressed(self.v[x]) {
                    self.pc += 2;
                },
                // ExA1 - SKNP Vx
                0xA1 => if !self.controls.is_key_pressed(self.v[x]) {
                    self.pc += 2;
                },
                _ => {}
            },
            0xF => match instr & 0xFF {
                // Fx07 - LD Vx, DT
                0x07 => self.v[x] = self.delay_timer,
                // Fx0A - LD Vx, K
                0x0A => {
                    self.is_paused = true;
                    self.controls.on_key_press(|key| {
                        self.v[x] = key;
                        self.is_paused = false;
                    });
                },
                // Fx15 - LD DT, Vx
                0x15 => self.delay_timer = self.v[x],
                // Fx18 - LD ST, Vx
                0x18 => self.sound_timer = self.v[x],
                // Fx1E - ADD I, Vx
                0x1E => self.i += self.v[x] as u16,
                // Fx29 - LD F, Vx
                0x29 => self.i = 5 * self.v[x] as u16,
                // Fx33 - LD B, Vx
                0x33 => {
                    self.memory[self.i as usize] = self.v[x] as u16 / 100;
                    self.memory[self.i as usize + 1] = (self.v[x] as u16 % 100) / 10;
                    self.memory[self.i as usize + 2] = self.v[x] as u16 % 10;
                },
                // Fx55 - LD [I], Vx
                0x55 => for i in 0..=x {
                    self.memory[self.i as usize + i] = self.v[i] as u16;
                },
                // Fx65 - LD Vx, [I]
                0x65 => for i in 0..=x {
                    self.v[i] = self.memory[self.i as usize + i] as u8;
                },
                _ => {}
            },
            _ => {}
        }
    }
}
