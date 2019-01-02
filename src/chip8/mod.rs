mod cpu;
mod graphics;
mod memory;
mod keypad;

use self::cpu::CPU;
use self::graphics::Graphics;
use self::memory::Memory;

use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::result;

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
    }

    pub fn get_mem(&self, addr: u16) -> u16 {
        self.memory.mem_read(addr)
    }

	 /*
	  * FIXME: Rewrite it with iterator.
	  *
	  */
    pub fn load_game(&mut self, file_path: &str) -> io::Result<()> {
        let file = File::open(file_path).unwrap();
        let filebytes: Vec<u8> = file.bytes().map(|readbyte| readbyte.unwrap()).collect();
		let mut j: usize = 0;
        for i in 0..self.memory.mem_size() {
            let msb: u8 = filebytes[j];
			let lsb: u8 = filebytes[j + 1];
			j = j + 2;
            self.memory.mem_write(self.cpu.pc + i as u16, ((msb as u16) << 8) | (lsb as u16));
        }
        Ok(())
    }

    pub fn emulate_cycle(&mut self) -> Result<(), &'static str> {
        self.cpu.execute(&mut self.memory)
    }
}
