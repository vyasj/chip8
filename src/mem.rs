use std::fs::File;
use std::io::BufReader;
use std::io::Read;

pub struct Memory {
    pub ram: Vec<u8>,
    pub stack: Vec<u16>,
}

impl Memory {
    pub fn init() -> Memory {
        let ret = Memory {
            ram: vec![0x00; 4096],
            stack: vec![0x0000; 16],
        };

        ret
    }

    pub fn load_font(&mut self) {
        let font = [
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
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];

        self.ram[0x050..0x0A0].copy_from_slice(&font);
    }

    pub fn load_rom(&mut self, filename: &str) {
        let mut ram_addr: usize = 0x200;
        let mut filepath: String = String::from("./roms/");
        filepath.push_str(filename);

        let buffer = BufReader::new(File::open(filepath).unwrap());

        for byte_or_error in buffer.bytes() {
            let byte = byte_or_error.unwrap();
            self.ram[ram_addr] = byte;
            ram_addr = ram_addr + 1;
        }
    }
}
