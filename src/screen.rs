pub struct Screen {

}

impl Screen {
    pub fn new() -> Self {
        Self {

        }
    }

    pub fn clear(&self) {
        println!("clear");
    }

    pub fn draw_pixel(&mut self, x: u16, y: u16) -> bool {
        false
    }
}
