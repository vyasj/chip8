use chip8::mem::Memory;
use chip8::disp::Display;
use chip8::reg::Registers;

fn main() {
    println!("Hello, CHIP-8!");

    println!("Initializing memory...");
    let mut memory = Memory::init();

    println!("Loading fonts...");
    memory.load_font();
}
