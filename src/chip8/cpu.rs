//chip8/cpu.rs

pub struct CPU {
    opcode : u16,
    sp : u16
}

impl CPU {
    pub fn new() -> CPU {
        CPU { opcode: 0x0, sp: 0x0 }
    }
}
