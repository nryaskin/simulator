mod chip8;

use chip8::Chip8;

fn main() {
    let running = true;
    let mut chip: Chip8 = Chip8::build_chip();
    chip.init();
    assert!(chip.load_game("pong").is_ok());

    while running {
        chip.emulate_cycle();
    }
}
