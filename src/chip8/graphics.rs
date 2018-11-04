pub struct Graphics {
    gfx: [u8; 64 * 32],
}

impl Graphics {
    pub fn new() -> Graphics{
        Graphics {gfx: [0; 64 * 32]}
    }
}
