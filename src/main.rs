use chip8::cpu::CPU;

fn main() {
    println!("Hello, CHIP-8!");

    println!("Initializing cpu...");
    let mut cpu = CPU::init();

    println!("Loading fonts...");
    cpu.load_font();
}
