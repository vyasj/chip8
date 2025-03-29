use chip8::mem::Memory;
use chip8::disp::Display;
use chip8::reg::Registers;

use std::env;

fn main() {
    println!("Hello, CHIP-8!");
    let filename = env::args()
        .nth(1)
        .expect("Expected a single command line argument");

    println!("Initializing memory...");
    let mut memory = Memory::init();

    println!("Loading fonts...");
    memory.load_font();

    println!("Loading rom...");
    memory.load_rom(&filename);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fonts() {
        let mut memory = Memory::init();
        memory.load_font();

        let exp_result = [
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

        assert_eq!(memory.ram[0x050..0x0A0], exp_result);
    }

    #[test]
    fn test_rom1() {
        let filename = "test_opcode.ch8";

        let mut memory = Memory::init();
        memory.load_font();

        memory.load_rom(filename);

        assert!(memory.ram[0x200] != 0);
    }
}