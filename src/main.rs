use chip8::mem::Memory;

struct Registers {
    pc: u16,
    ir: u16,
    v0: u8,
    v1: u8,
    v2: u8,
    v3: u8,
    v4: u8,
    v5: u8,
    v6: u8,
    v7: u8,
    v8: u8,
    v9: u8,
    va: u8,
    vb: u8,
    vc: u8,
    vd: u8,
    ve: u8,
    vf: u8
}

fn main() {
    println!("Hello, CHIP-8!");

    println!("Initializing memory...");
    let mut memory = Memory::init();

    println!("Loading fonts...");
    memory.load_font();
}
