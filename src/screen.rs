const WIDTH: usize = 32;
const HEIGHT: usize = 64;

pub struct Screen {
    screen: [[u8; HEIGHT]; WIDTH]
}

impl Screen {
    pub fn new() -> Self {
        Self {
            screen: [[0; HEIGHT]; WIDTH]
        }
    }

    pub fn draw(&mut self) {

    }

    pub fn draw_pixel(&mut self, x: u16,  y: u16) -> bool {
        let x = (x % WIDTH as u16) as usize;
        let y = (y % HEIGHT as u16) as usize;

        self.screen[y][x] ^= 1;

        self.screen[y][x] != 1
    }

    pub fn clear(&mut self) {
        self.screen = [[0; HEIGHT]; WIDTH];
    }


}
