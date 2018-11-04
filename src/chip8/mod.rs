mod cpu;
mod graphics;
mod memory;

use self::cpu::CPU;
use self::graphics::Graphics;
use self::memory::Memory;

pub struct Chip8 {
    cpu: CPU,
    memory: Memory,
    gr: Graphics,
}

impl Chip8 {
    pub fn build_chip() -> Chip8 {
        Chip8 {
            cpu: CPU::new(),
            memory: Memory::new(),
            gr: Graphics::new(),
        }
    }
    pub fn init(&mut self) {
        self.memory.mem_init();
        self.memory.mem_write(0x1, 0x15);
    }

    pub fn get_mem(&self, addr: u16) -> u16 {
        self.memory.mem_read(addr)
    }
}
