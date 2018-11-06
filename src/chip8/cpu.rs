//chip8/cpu.rs

const CHIP8_REGISTER_COUNT: usize = 16;
const CHIP8_STACK_SIZE: usize = 16;

pub struct CPU {
    opcode : u16,
    index: u16,
    pub pc : u16,
    registers: [u8; CHIP8_REGISTER_COUNT], 
    stack: [u16; CHIP8_STACK_SIZE],
    sp: u16,
    delay_timer: u8,
    sound_timer: u8,
}

impl CPU {
    pub fn new() -> CPU {
        CPU { 
                opcode :     0x0,
                index:       0x0,
                pc :         0x200,
                registers:  [0x0; CHIP8_REGISTER_COUNT],
                stack:      [0x0; CHIP8_STACK_SIZE],
                sp:          0x0,
                delay_timer: 0x0,
                sound_timer: 0x0,
        }
    }
}
