pub struct Controls {

}

impl Controls {
    pub fn is_key_pressed(&self, key: u8) -> bool {
        false
    }

    pub fn on_key_press<F: FnMut(u8)>(&self, mut f: F) {
        // get key
        let key = 0;
        f(key);
    }
}
