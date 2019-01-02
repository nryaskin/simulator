//chip8/cpu.rs
use super::memory::Memory;
use std::result;

const CHIP8_REGISTER_COUNT:     usize   = 16;
const CHIP8_STACK_SIZE:         usize   = 16;
const CHIP8_CODE_TYPE_MASK:     u16     = 0xF000;
const CHIP8_SUBCODE_TYPE_MASK:  u16     = 0x000F;
const CHIP8_FLOW_ADDR_MASK:     u16     = 0x0FFF;
const CHIP8_REGISTER_X_MASK:    u16     = 0x0F00;
const CHIP8_REGISTER_Y_MASK:    u16     = 0x00F0;
const CHIP8_NN_MASK:            u16     = 0x00FF;       

pub struct CPU {
    //pub opcode : u16,
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
                //opcode :     0x0,
                index:       0x0,
                pc :         0x200,
                registers:  [0x0; CHIP8_REGISTER_COUNT],
                stack:      [0x0; CHIP8_STACK_SIZE],
                sp:          0x0,
                delay_timer: 0x0,
                sound_timer: 0x0,
        }
    }

    pub fn execute(&mut self, memory: & mut Memory) -> Result<(), &'static str> {
        let opcode = memory.mem_read(self.pc); 

        match opcode & CHIP8_CODE_TYPE_MASK {
        0x0 =>  match opcode & CHIP8_SUBCODE_TYPE_MASK {
                0x0000 => {
                    self.disp_clear(memory);
                    Ok(())
                },
                0x000E => {
                    self.subrouting_return(); 
                    Ok(())
                },
                _ => Err(""),
        },
        0x1000 => {
            self.goto(opcode & CHIP8_FLOW_ADDR_MASK);
            Ok(())
        },
        0x2000 => {
            self.subrouting_call(opcode & CHIP8_FLOW_ADDR_MASK);
            Ok(())
        },
        0x3000 => {
            self.if_equal_skip(((opcode & CHIP8_REGISTER_X_MASK) >> 8) as usize,
                               (opcode & CHIP8_NN_MASK) as u8);
            Ok(())
        },
        0x4000 => {
            self.if_not_equal_skip(((opcode & CHIP8_REGISTER_X_MASK) >> 8) as usize,
                                   (opcode & CHIP8_NN_MASK) as u8);
            Ok(())
        },
        0x5000 => {
            self.if_x_y_equal_skip(((opcode & CHIP8_REGISTER_X_MASK) >> 8) as usize,
                                   (opcode & CHIP8_REGISTER_Y_MASK >> 4) as usize);
            Ok(())
        },
        0x6000 => {
            self.reg_set(((opcode & CHIP8_REGISTER_X_MASK) >> 8) as usize,
                         (opcode & CHIP8_NN_MASK) as u8);
            Ok(())
        },
        0x7000 => {
            self.reg_add(((opcode & CHIP8_REGISTER_X_MASK) >> 8) as usize,
                         (opcode & CHIP8_NN_MASK) as u8);
            Ok(())
        },
        _ => Err(""),
        }
    }

    fn goto(&mut self, addr: u16) {
        self.pc = addr;
    }

    fn disp_clear(&self, memory: & mut Memory) {
        
    }

    fn subrouting_return(& mut self) {
        assert!(self.sp != 0);
        self.sp = self.sp - 1;
        self.pc = self.stack[self.sp as usize];
    }

    fn subrouting_call(& mut self, addr: u16) {
        self.stack[self.sp as usize] = self.pc;
        self.sp = self.sp + 1;
        self.pc = addr;
    }
    
    fn if_equal_skip(& mut self, r_x: usize, nn: u8) {
        if self.registers[r_x] == nn {
            self.pc_next();
        }
        self.pc_next(); 
    }
 
    fn if_x_y_equal_skip(& mut self, r_x: usize, r_y: usize) {
        if self.registers[r_x] == self.registers[r_y] {
            self.pc_next();
        }
        self.pc_next();
    }

    fn if_not_equal_skip(& mut self, r_x: usize, nn: u8) {
        if self.registers[r_x] != nn {
            self.pc_next();
        }
        self.pc_next();
    }

    fn reg_set(&mut self, r_x: usize, nn: u8) {
        self.registers[r_x] = nn;
        self.pc_next();
    }

    fn reg_add(&mut self, r_x: usize, nn: u8) {
        self.registers[r_x] += nn;
        self.pc_next();
    }

    fn pc_next(& mut self) {
        self.pc = self.pc + 1
    }
}
