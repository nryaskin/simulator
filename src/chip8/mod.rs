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

const CHIP8_FONTSET: [u8; 80] =
[ 
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
        for (i, &data) in CHIP8_FONTSET.iter().enumerate() {
            self.memory.mem_write(i as u16, data);
        }
    }

    pub fn get_mem(&self, addr: u16) -> u8 {
        self.memory.mem_read(addr)
    }

	 /*
	  * FIXME: Rewrite it with iterator.
	  *
	  */
    pub fn load_game(&mut self, file_path: &str) -> io::Result<()> {
        let file = File::open(file_path).unwrap();
        let filebytes: Vec<u8> = file.bytes().map(|readbyte| readbyte.unwrap()).collect();
		for (i, &data) in filebytes.iter().enumerate() {
            self.memory.mem_write(self.cpu.pc + i as u16, data);
        }
        Ok(())
    }

    pub fn emulate_cycle(&mut self) -> Result<(), &'static str> {
        self.cpu.execute(&mut self.memory, &mut self.gr)
    }
}
