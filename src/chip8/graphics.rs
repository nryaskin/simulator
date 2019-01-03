pub struct Graphics {
    pub gfx: [u8; 64 * 32],
    draw_flag: bool,
}

impl Graphics {
    pub fn new() -> Graphics{
        Graphics {gfx: [0; 64 * 32], draw_flag: false}
    }

    pub fn draw_flag_set(&mut self) {
        self.draw_flag = true;
    }
}
