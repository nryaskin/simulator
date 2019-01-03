const MEMORY_SIZE: usize = 4_096;

pub struct Memory {
    data: [u8; MEMORY_SIZE],
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            data: [0; MEMORY_SIZE]
        }
    }
    pub fn mem_init(&mut self) {
        self.data = [0; MEMORY_SIZE];
    }

    pub fn mem_write(&mut self, addr: u16, value: u8) {
        self.data[addr as usize] = value;
    }

    pub fn mem_read(& self, addr: u16) -> u8 {
       self.data[addr as usize]
    }
    
    pub fn mem_size(& self) -> usize {
        self.data.len()
    }
}
