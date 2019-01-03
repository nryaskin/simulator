//chip8/cpu.rs
extern crate rand;

use super::memory::Memory;
use std::result;
use self::rand::prelude::*;

const CHIP8_REGISTER_COUNT:     usize   = 16;
const CHIP8_STACK_SIZE:         usize   = 16;
const CHIP8_CODE_TYPE_MASK:     u16     = 0xF000;
const CHIP8_SUBCODE_TYPE_MASK:  u16     = 0x000F;
const CHIP8_FLOW_ADDR_MASK:     u16     = 0x0FFF;
const CHIP8_REGISTER_X_MASK:    u16     = 0x0F00;
const CHIP8_REGISTER_Y_MASK:    u16     = 0x00F0;
const CHIP8_NN_MASK:            u16     = 0x00FF;       
const CHIP8_NNN_MASK:           u16     = 0x0FFF;       
const CHIP8_VF_REGISTER:        usize   = 0xF;
const CHIP8_V0_REGISTER:        usize   = 0x0;

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
        let opcode: u16 = (memory.mem_read(self.pc) as u16) << 8
                           | memory.mem_read(self.pc + 1) as u16; 

        let result = match opcode & CHIP8_CODE_TYPE_MASK {
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
        0x8000 => {
            let r_x: usize = ((opcode & CHIP8_REGISTER_X_MASK) >> 8) as usize;
            let r_y: usize = (opcode & CHIP8_REGISTER_Y_MASK >> 4) as usize;
            match opcode & CHIP8_SUBCODE_TYPE_MASK {
            0x0 =>  self.reg_assing(r_x, r_y),
            0x0001 => self.reg_assing(r_x, r_y),
            0x0002 => self.bit_or(r_x, r_y),
            0x0003 => self.bit_xor(r_x, r_y),
            0x0004 => self.math_add(r_x, r_y),
            0x0005 => self.math_sub(r_x, r_y),
            0x0006 => self.bit_right_shift(r_x, r_y),
            0x0007 => self.math_dif_sub(r_x, r_y),
            0x000E => self.bit_left_shift(r_x, r_y),
            _ => {},
            };
            Ok(())
        },
        0x9000 => {
            let r_x: usize = ((opcode & CHIP8_REGISTER_X_MASK) >> 8) as usize;
            let r_y: usize = (opcode & CHIP8_REGISTER_Y_MASK >> 4) as usize;
            self.if_not_x_y_equal_skip(r_x, r_y);
            Ok(())
        },
        0xA000 => {
            self.index_set(opcode & CHIP8_NNN_MASK); 
            Ok(())
        },
        0xB000 => {
            self.jump(opcode & CHIP8_NNN_MASK); 
            Ok(())
        },
        0xC000 => {
            let r_x = ((opcode & CHIP8_REGISTER_X_MASK) >> 8) as usize;
            let nn = (opcode & (CHIP8_NN_MASK)) as u8;
            self.bitwise_random(r_x, nn);
            Ok(())
        },
        0xD000 => {
            Err("Not implementedyet")
        },
        0xE000 => {
            Err("Not implementedyet")
        },
        0xF000 => {
            let r_x = ((opcode & CHIP8_REGISTER_X_MASK) >> 8) as usize;
            match opcode & 0x00FF {
            0x0007 => self.delay_get(r_x),
            0x000A => return Err(""),
            0x0015 => self.delay_set(r_x),
            0x0018 => self.sound_timer_set(r_x),
            0x001E => self.index_add(r_x),
            0x0029 => return Err(""),
            0x0033 => self.bcd_set(memory, r_x),
            0x0055 => self.reg_dump(memory, r_x),
            0x0065 => self.reg_load(memory, r_x),
            _ => (),
            };
            Ok(())
        }
        _ => Err(""),
        };

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
               //TODO: Add sound callback. 
            }
            self.sound_timer -= 1;
        }
        return result;
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
    
    fn if_not_x_y_equal_skip(& mut self, r_x: usize, r_y: usize) {
        if self.registers[r_x] != self.registers[r_y] {
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

    /*
     * TODO: Maybe later use operator overloading.
     */
    fn reg_set(&mut self, r_x: usize, nn: u8) {
        self.registers[r_x] = nn;
        self.pc_next();
    }

    fn reg_add(&mut self, r_x: usize, nn: u8) {
        self.registers[r_x] += nn;
        self.pc_next();
    }

    fn reg_assing(&mut self, r_x: usize, r_y: usize) {
        self.registers[r_x] = self.registers[r_y];
        self.pc_next();
    }

    fn bit_or(& mut self, r_x: usize, r_y: usize) {
        self.registers[r_x] |= self.registers[r_y];
        self.pc_next();
    }

    fn bit_and(& mut self, r_x: usize, r_y: usize) {
        self.registers[r_x] &= self.registers[r_y];
        self.pc_next();
    }
    
    fn bit_xor(& mut self, r_x: usize, r_y: usize) {
        self.registers[r_x] ^= self.registers[r_y];
        self.pc_next();
    }
    
    fn math_add(& mut self, r_x: usize, r_y: usize) {
        let (val, carry) = self.registers[r_x].overflowing_add(self.registers[r_y]);
        self.registers[CHIP8_VF_REGISTER] = if carry {
                1
            } else {
                0
            };
        self.registers[r_x] = val;
        self.pc_next();
    }
    
    fn math_sub(& mut self, r_x: usize, r_y: usize) {
        let (val, borrow) = self.registers[r_x].overflowing_sub(self.registers[r_y]);
        self.registers[CHIP8_VF_REGISTER] = if borrow {
                1
            } else {
                0
            };
        self.registers[r_x] = val;
        self.pc_next();
    }
    
    fn bit_right_shift(& mut self, r_x: usize, r_y: usize) {
        self.registers[CHIP8_VF_REGISTER] = self.registers[CHIP8_VF_REGISTER] & 0x01;
        self.registers[r_x] >>= 1;
        self.pc_next();
    }

    fn math_dif_sub(& mut self, r_x: usize, r_y: usize) {
        let (val, borrow) = self.registers[r_y].overflowing_sub(self.registers[r_x]);
        self.registers[CHIP8_VF_REGISTER] = if borrow {
                1
            } else {
                0
            };
        self.registers[r_x] = val;
        self.pc_next();
    }

    fn bit_left_shift(& mut self, r_x: usize, r_y: usize) {
        self.registers[CHIP8_VF_REGISTER] = self.registers[CHIP8_VF_REGISTER] & 0x80;
        self.registers[r_x] <<= 1;
        self.pc_next();
    }

    fn index_set(& mut self, nnn: u16) {
        self.index = nnn;
        self.pc_next();
    }

    fn jump(& mut self, nnn: u16) {
        self.pc = self.registers[0] as u16 + nnn;
    }

    fn bitwise_random(& mut self, r_x: usize, nn: u8) {
        self.registers[r_x] = self::rand::random::<u8>() & nn;
        self.pc_next();
    }

    fn delay_get(&mut self, r_x: usize) {
        self.registers[r_x] = self.delay_timer;
        self.pc_next();
    }

    fn delay_set(&mut self, r_x: usize) {
        self.delay_timer = self.registers[r_x];
        self.pc_next();
    }
    
    fn sound_timer_set(&mut self, r_x: usize) {
        self.sound_timer = self.registers[r_x];
        self.pc_next();
    }

    fn index_add(&mut self, r_x: usize) {
        self.index += self.registers[r_x] as u16;
        self.pc_next();
    }

    fn bcd_set(&mut self, mem: &mut Memory, r_x: usize) {
        mem.mem_write(self.index, self.registers[r_x] / 100);
        mem.mem_write(self.index + 1, self.registers[r_x] % 100 / 10);
        mem.mem_write(self.index + 2, self.registers[r_x] % 10);
        self.pc_next();
    }

    fn reg_dump(&mut self, mem: &mut Memory, r_x: usize) {
        for (i, &data) in self.registers.iter().enumerate().take(r_x + 1) {
            mem.mem_write(self.index + i as u16 , data); 
        }
    }

    fn reg_load(&mut self, mem: &mut Memory, r_x: usize) {
        for i in 0..r_x + 1 {
            self.registers[i] = mem.mem_read(self.index + i as u16); 
        }
    }

    fn pc_next(&mut self) {
        self.pc = self.pc + 2;
    }
}
