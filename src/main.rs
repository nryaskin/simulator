mod chip8;

use chip8::Chip8;

fn main() {
    let mut running: bool = false;
    let mut chip: Chip8 = Chip8::build_chip();
    chip.init();
/*
    while running {
        
    }
*/
    println!("Opcode is: {:X}", chip.get_mem(0x1));
}
