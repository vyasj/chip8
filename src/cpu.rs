use std::fs::File;
use std::io::BufReader;
use std::io::Read;

use rand::Rng;

pub enum JPType {
    Addr(u16),
    FromV0(u16),
}

pub enum SEType {
    Byte(u8),
    Reg(u8),
}

pub enum LDType {
    Byte(u8),
    Reg(u8),
    Addr(u16),
    FromDT,
    KeyPress,
    ToDT,
    ToST,
    F,
    B,
    ToI,
    FromI,
}

pub enum AddType {
    Byte(u8),
    Reg(u8),
    I,
}

pub enum Instruction {
    CLS,
    RET,
    SYS(u16),
    JP(JPType),
    CALL(u16),
    SE(u8, SEType),
    SNE(u8, SEType),
    LD(u8, LDType),
    ADD(u8, AddType),
    OR(u8, u8),
    AND(u8, u8),
    XOR(u8, u8),
    SUB(u8, u8),
    SHR(u8, u8),
    SUBN(u8, u8),
    SHL(u8, u8),
    RND(u8, u8),
    DRW(u8, u8, u8),
    SKP(u8),
    SKNP(u8),
    RAW0,
}

impl Instruction {
    pub fn decode(instruction: u16) -> Option<Self> {
        let nibbles = (
            (0xF000 & instruction) >> 12,
            (0x0F00 & instruction) >> 8,
            (0x00F0 & instruction) >> 4,
            0x000F & instruction,
        );

        let nnn = 0x0FFF & instruction;
        let kk = (0x00FF & instruction) as u8;
        let n = (0x000F & instruction) as u8;
        let x = ((0x0F00 & instruction) >> 8) as u8;
        let y = ((0x00F0 & instruction) >> 4) as u8;

        Some(match nibbles {
            (0x0, 0x0, 0x0, 0x0) => Instruction::RAW0,
            (0x0, 0x0, 0xE, 0x0) => Instruction::CLS,
            (0x0, 0x0, 0xE, 0xE) => Instruction::RET,
            (0x0, _, _, _) => Instruction::SYS(nnn),
            (0x1, _, _, _) => Instruction::JP(JPType::Addr(nnn)),
            (0x2, _, _, _) => Instruction::CALL(nnn),
            (0x3, _, _, _) => Instruction::SE(x, SEType::Byte(kk)),
            (0x4, _, _, _) => Instruction::SNE(x, SEType::Byte(kk)),
            (0x5, _, _, 0x0) => Instruction::SE(x, SEType::Reg(y)),
            (0x6, _, _, _) => Instruction::LD(x, LDType::Byte(kk)),
            (0x7, _, _, _) => Instruction::ADD(x, AddType::Byte(kk)),
            (0x8, _, _, 0x0) => Instruction::LD(x, LDType::Reg(y)),
            (0x8, _, _, 0x1) => Instruction::OR(x, y),
            (0x8, _, _, 0x2) => Instruction::AND(x, y),
            (0x8, _, _, 0x3) => Instruction::XOR(x, y),
            (0x8, _, _, 0x4) => Instruction::ADD(x, AddType::Reg(y)),
            (0x8, _, _, 0x5) => Instruction::SUB(x, y),
            (0x8, _, _, 0x6) => Instruction::SHR(x, y),
            (0x8, _, _, 0x7) => Instruction::SUBN(x, y),
            (0x8, _, _, 0xE) => Instruction::SHL(x, y),
            (0x9, _, _, 0x0) => Instruction::SNE(x, SEType::Reg(y)),
            (0xA, _, _, _) => Instruction::LD(0, LDType::Addr(nnn)),
            (0xB, _, _, _) => Instruction::JP(JPType::FromV0(nnn)),
            (0xC, _, _, _) => Instruction::RND(x, kk),
            (0xD, _, _, _) => Instruction::DRW(x, y, n),
            (0xE, _, 0x9, 0xE) => Instruction::SKP(x),
            (0xE, _, 0xA, 0x1) => Instruction::SKNP(x),
            (0xF, _, 0x0, 0x7) => Instruction::LD(x, LDType::FromDT),
            (0xF, _, 0x0, 0xA) => Instruction::LD(x, LDType::KeyPress),
            (0xF, _, 0x1, 0x5) => Instruction::LD(x, LDType::ToDT),
            (0xF, _, 0x1, 0x8) => Instruction::LD(x, LDType::ToST),
            (0xF, _, 0x1, 0xE) => Instruction::ADD(x, AddType::I),
            (0xF, _, 0x2, 0x9) => Instruction::LD(x, LDType::F),
            (0xF, _, 0x3, 0x3) => Instruction::LD(x, LDType::B),
            (0xF, _, 0x5, 0x5) => Instruction::LD(x, LDType::ToI),
            (0xF, _, 0x6, 0x5) => Instruction::LD(x, LDType::FromI),
            _ => return None,
        })
    }

    pub fn print_name(ins: &Self) {
        match ins {
            Self::RAW0 => println!("RAW0"),
            Self::CLS => println!("CLS"),
            Self::RET => println!("RET"),
            Self::SYS(nnn) => println!("SYS {:#x}", nnn),
            Self::JP(JPType::Addr(nnn)) => println!("JP {:#x}", nnn),
            Self::JP(JPType::FromV0(nnn)) => println!("JP V0 + {:#x}", nnn),
            Self::CALL(nnn) => println!("CALL {:#x}", nnn),
            Self::SE(x, SEType::Byte(kk)) => println!("SE V{:#x}, {:#x}", x, kk),
            Self::SE(x, SEType::Reg(y)) => println!("SE V{:#x}, V{:#x}", x, y),
            Self::SNE(x, SEType::Byte(kk)) => println!("SNE V{:#x}, {:#x}", x, kk),
            Self::SNE(x, SEType::Reg(y)) => println!("SNE V{:#x}, V{:#x}", x, y),
            Self::LD(_, LDType::Addr(nnn)) => println!("LD I, {:#x}", nnn),
            Self::LD(x, LDType::B) => println!("LD B, V{:#x}", x),
            Self::LD(x, LDType::Byte(kk)) => println!("LD V{:#x}, {:#x}", x, kk),
            Self::LD(x, LDType::F) => println!("LD F, V{:#x}", x),
            Self::LD(x, LDType::FromDT) => println!("LD V{:#x}, DT", x),
            Self::LD(x, LDType::FromI) => println!("LD V{:#x}, [I]", x),
            Self::LD(x, LDType::KeyPress) => println!("LD V{:#x}, K", x),
            Self::LD(x, LDType::Reg(y)) => println!("LD V{:#x}, V{:#x}", x, y),
            Self::LD(x, LDType::ToDT) => println!("LD DT, V{:#x}", x),
            Self::LD(x, LDType::ToI) => println!("LD [I], V{:#x}", x),
            Self::LD(x, LDType::ToST) => println!("LD ST, V{:#x}", x),
            Self::ADD(x, AddType::Byte(kk)) => println!("ADD V{:#x}, {:#x}", x, kk),
            Self::ADD(x, AddType::I) => println!("ADD V{:#x}, I", x),
            Self::ADD(x, AddType::Reg(y)) => println!("ADD V{:#x}, V{:#x}", x, y),
            Self::OR(x, y) => println!("OR V{:#x}, V{:#x}", x, y),
            Self::AND(x, y) => println!("AND V{:#x}, V{:#x}", x, y),
            Self::XOR(x, y) => println!("XOR V{:#x}, V{:#x}", x, y),
            Self::SUB(x, y) => println!("SUB V{:#x}, V{:#x}", x, y),
            Self::SHR(x, y) => println!("SHR V{:#x} {{, V{:#x}}}", x, y),
            Self::SUBN(x, y) => println!("SUBN V{:#x}, V{:#x}", x, y),
            Self::SHL(x, y) => println!("SHL V{:#x} {{, V{:#x}}}", x, y),
            Self::RND(x, kk) => println!("RND V{:#x}, {:#x}", x, kk),
            Self::DRW(x, y, n) => println!("DRW V{:#x}, V{:#x}, {:#x}", x, y, n),
            Self::SKP(x) => println!("SKP V{:#x}", x),
            Self::SKNP(x) => println!("SKNP V{:#x}", x),
        };
    }
}

pub struct Cpu {
    pub ram: Vec<u8>,
    pub stack: Vec<u16>,

    pub pc: u16,       // program counter
    pub ir: u16,       // index register
    pub vx: Vec<u8>,   // V-registers
    pub st: u8,        // sound timer
    pub dt: u8,        // delay timer
    pub sp: u8,        // stack pointer
    pub kp: Vec<bool>, // key pressed

    pub height: u8,
    pub width: u8,
    pub screen: Vec<bool>,
}

impl Cpu {
    pub fn init() -> Cpu {
        let mut ret = Cpu {
            ram: vec![0x00; 4096],
            stack: vec![0x0000; 16],

            pc: 0x200,
            ir: 0,
            vx: vec![0; 16],
            st: 60,
            dt: 60,
            sp: 0,
            kp: vec![false; 16],

            height: 32,
            width: 64,
            screen: vec![false; 64 * 32],
        };

        ret.load_font();

        ret
    }

    fn load_font(&mut self) {
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
            ram_addr += 1;
        }
    }

    pub fn print_ram(&mut self) {
        let start_pc = self.pc;
        self.pc = 0x200;

        loop {
            if self.pc as usize >= self.ram.len() {
                break;
            }

            let b1 = self.ram[self.pc as usize] as u16;
            let b2 = self.ram[(self.pc + 1) as usize] as u16;
            print!("{:#06x}\t({:#06x})\t", self.pc, (b1 << 8) | b2);

            let bytes = self.fetch();
            if bytes == 0x0000 {
                break;
            }

            let ins = Instruction::decode(bytes);

            if ins.is_some() {
                Instruction::print_name(&(ins.unwrap()));
            }
        }

        self.pc = start_pc;
    }

    pub fn dump_state(&self) {
        println!("\nregisters:");
        for i in 0..self.vx.len() {
            println!("V{:#x}: {:#x}", i, self.vx[i]);
        }

        println!("\nstack:");
        for i in 0..self.stack.len() {
            println!("V{:#x}: {:#x}", i, self.vx[i]);
        }

        println!("\nprogram counter: {:#x}", self.pc);
        println!("index register: {:#x}", self.ir);
    }

    pub fn draw(&self, frame: &mut [u8]) {
        for (idx, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let pixel_on = self.screen[idx];

            let rgba = if !pixel_on {
                [0x00, 0x00, 0x00, 0xFF]
            } else {
                [0xFF, 0xFF, 0xFF, 0xFF]
            };

            pixel.copy_from_slice(&rgba);
        }
    }

    pub fn fetch(&mut self) -> u16 {
        let n1: u16 = self.ram[self.pc as usize] as u16;
        let n2: u16 = self.ram[(self.pc + 1) as usize] as u16;
        self.pc += 2;

        (n1 << 8) | n2
    }

    pub fn execute(&mut self, instruction: Instruction) -> Option<u8> {
        match instruction {
            Instruction::RAW0 => None,
            Instruction::CLS => self.op_00e0(),
            Instruction::RET => self.op_00ee(),
            Instruction::SYS(nnn) => self.op_0nnn(nnn),
            Instruction::JP(JPType::Addr(nnn)) => self.op_1nnn(nnn),
            Instruction::CALL(nnn) => self.op_2nnn(nnn),
            Instruction::SE(x, SEType::Byte(kk)) => self.op_3xkk(x, kk),
            Instruction::SNE(x, SEType::Byte(kk)) => self.op_4xkk(x, kk),
            Instruction::SE(x, SEType::Reg(y)) => self.op_5xy0(x, y),
            Instruction::LD(x, LDType::Byte(kk)) => self.op_6xkk(x, kk),
            Instruction::ADD(x, AddType::Byte(kk)) => self.op_7xkk(x, kk),
            Instruction::LD(x, LDType::Reg(y)) => self.op_8xy0(x, y),
            Instruction::OR(x, y) => self.op_8xy1(x, y),
            Instruction::AND(x, y) => self.op_8xy2(x, y),
            Instruction::XOR(x, y) => self.op_8xy3(x, y),
            Instruction::ADD(x, AddType::Reg(y)) => self.op_8xy4(x, y),
            Instruction::SUB(x, y) => self.op_8xy5(x, y),
            Instruction::SHR(x, y) => self.op_8xy6(x, y),
            Instruction::SUBN(x, y) => self.op_8xy7(x, y),
            Instruction::SHL(x, y) => self.op_8xye(x, y),
            Instruction::SNE(x, SEType::Reg(y)) => self.op_9xy0(x, y),
            Instruction::LD(_, LDType::Addr(nnn)) => self.op_annn(nnn),
            Instruction::JP(JPType::FromV0(nnn)) => self.op_bnnn(nnn),
            Instruction::RND(x, kk) => self.op_cxkk(x, kk),
            Instruction::DRW(x, y, n) => self.op_dxyn(x, y, n),
            Instruction::SKP(x) => self.op_ex9e(x),
            Instruction::SKNP(x) => self.op_exa1(x),
            Instruction::LD(x, LDType::FromDT) => self.op_fx07(x),
            Instruction::LD(x, LDType::KeyPress) => self.op_fx0a(x),
            Instruction::LD(x, LDType::ToDT) => self.op_fx15(x),
            Instruction::LD(x, LDType::ToST) => self.op_fx18(x),
            Instruction::ADD(x, AddType::I) => self.op_fx1e(x),
            Instruction::LD(x, LDType::F) => self.op_fx29(x),
            Instruction::LD(x, LDType::B) => self.op_fx33(x),
            Instruction::LD(x, LDType::ToI) => self.op_fx55(x),
            Instruction::LD(x, LDType::FromI) => self.op_fx65(x),
        }
    }

    fn op_00e0(&mut self) -> Option<u8> {
        // CLS
        for x in &mut self.screen {
            *x = false;
        }

        None
    }

    fn op_00ee(&mut self) -> Option<u8> {
        // RET
        self.pc = self.stack[self.sp as usize];
        self.sp -= 1;

        None
    }

    fn op_0nnn(&mut self, nnn: u16) -> Option<u8> {
        // SYS addr
        println!("Instruction {:#x} is for a system call.", nnn);

        None
    }

    fn op_1nnn(&mut self, nnn: u16) -> Option<u8> {
        // JP addr
        self.pc = nnn;

        None
    }

    fn op_2nnn(&mut self, nnn: u16) -> Option<u8> {
        // CALL addr
        self.sp += 1;
        self.stack[self.sp as usize] = self.pc;
        self.pc = nnn;

        None
    }

    fn op_3xkk(&mut self, x: u8, kk: u8) -> Option<u8> {
        // SE x, kk
        if self.vx[x as usize] == kk {
            self.pc += 2;
        }

        None
    }

    fn op_4xkk(&mut self, x: u8, kk: u8) -> Option<u8> {
        // SNE x, kk
        if self.vx[x as usize] != kk {
            self.pc += 2;
        }

        None
    }

    fn op_5xy0(&mut self, x: u8, y: u8) -> Option<u8> {
        // SE x, y
        if self.vx[x as usize] == self.vx[y as usize] {
            self.pc += 2;
        }

        None
    }

    fn op_6xkk(&mut self, x: u8, kk: u8) -> Option<u8> {
        // LD x, kk
        self.vx[x as usize] = kk;

        None
    }

    fn op_7xkk(&mut self, x: u8, kk: u8) -> Option<u8> {
        // ADD x, kk
        self.vx[x as usize] = self.vx[x as usize].wrapping_add(kk);

        None
    }

    fn op_8xy0(&mut self, x: u8, y: u8) -> Option<u8> {
        // LD x, y
        self.vx[x as usize] = self.vx[y as usize];

        None
    }

    fn op_8xy1(&mut self, x: u8, y: u8) -> Option<u8> {
        // OR x, y
        self.vx[x as usize] |= self.vx[y as usize];

        None
    }

    fn op_8xy2(&mut self, x: u8, y: u8) -> Option<u8> {
        // AND x, y
        self.vx[x as usize] &= self.vx[y as usize];

        None
    }

    fn op_8xy3(&mut self, x: u8, y: u8) -> Option<u8> {
        // XOR x, y
        self.vx[x as usize] ^= self.vx[y as usize];

        None
    }

    fn op_8xy4(&mut self, x: u8, y: u8) -> Option<u8> {
        // ADD x, y
        let sum: u16 = (self.vx[x as usize] as u16) + (self.vx[y as usize] as u16);

        if sum & 0xFF00 > 0 {
            self.vx[0xF] = 1;
        }

        self.vx[x as usize] = sum as u8;

        None
    }

    fn op_8xy5(&mut self, x: u8, y: u8) -> Option<u8> {
        // SUB x, y
        self.vx[0xF] = if self.vx[x as usize] > self.vx[y as usize] {
            1
        } else {
            0
        };

        self.vx[x as usize] = if self.vx[0xF] == 1 {
            self.vx[x as usize] - self.vx[y as usize]
        } else {
            self.vx[y as usize] - self.vx[x as usize]
        };

        None
    }

    fn op_8xy6(&mut self, x: u8, _y: u8) -> Option<u8> {
        // SHR x {, y}
        if self.vx[x as usize].trailing_ones() > 1 {
            self.vx[0xF] = 1
        } else {
            self.vx[0xF] = 0
        };
        self.vx[x as usize] = self.vx[x as usize] >> 1;

        None
    }

    fn op_8xy7(&mut self, x: u8, y: u8) -> Option<u8> {
        // SUBN x, y
        self.vx[0xF] = if self.vx[y as usize] > self.vx[x as usize] {
            1
        } else {
            0
        };

        self.vx[x as usize] = if self.vx[0xF] == 0 {
            self.vx[x as usize] - self.vx[y as usize]
        } else {
            self.vx[y as usize] - self.vx[x as usize]
        };

        None
    }

    fn op_8xye(&mut self, x: u8, _y: u8) -> Option<u8> {
        // SHR x {, y}
        if self.vx[x as usize].leading_ones() > 1 {
            self.vx[0xF] = 1
        } else {
            self.vx[0xF] = 0
        };
        self.vx[x as usize] = self.vx[x as usize] << 1;

        None
    }

    fn op_9xy0(&mut self, x: u8, y: u8) -> Option<u8> {
        // SNE x, y
        if self.vx[x as usize] != self.vx[y as usize] {
            self.pc += 2;
        }

        None
    }

    fn op_annn(&mut self, nnn: u16) -> Option<u8> {
        // LD I, addr
        self.ir = nnn;

        None
    }

    fn op_bnnn(&mut self, nnn: u16) -> Option<u8> {
        // JP V0, addr
        self.pc = (self.vx[0] as u16) + nnn;

        None
    }

    fn op_cxkk(&mut self, x: u8, kk: u8) -> Option<u8> {
        // RND x, kk
        let mut rng = rand::rng();
        self.vx[x as usize] = rng.random_range(0..=255) & kk;

        None
    }

    fn op_dxyn(&mut self, x: u8, y: u8, n: u8) -> Option<u8> {
        // DRW x, y, n
        self.vx[0xF] = 0;
        for i in 0..n {
            let sprite_byte: u8 = self.ram[(self.ir + i as u16) as usize];

            for j in 0..8 {
                if (sprite_byte << j) & 0x80 != 0 {
                    let counter_x = ((self.vx[x as usize] + j) % (self.width)) as usize;
                    let counter_y = ((self.vx[y as usize] + i) % (self.height)) as usize;
                    let screen_idx = (counter_y * self.width as usize) + counter_x;

                    if self.screen[screen_idx] {
                        self.vx[0xF] = 1;
                    }
                    self.screen[screen_idx] ^= true;
                }
            }
        }

        // returning 1 is understood as a call to draw, so the main() loop
        // knows to request the game window for a redraw
        Some(1)
    }

    fn op_ex9e(&mut self, x: u8) -> Option<u8> {
        // SKP x
        let key = self.vx[x as usize] as usize;
        if self.kp[key] {
            self.pc += 2;
        }

        None
    }

    fn op_exa1(&mut self, x: u8) -> Option<u8> {
        // SKNP x
        let key = self.vx[x as usize] as usize;
        if !self.kp[key] {
            self.pc += 2;
        }

        None
    }

    fn op_fx07(&mut self, x: u8) -> Option<u8> {
        // LD x, DT
        self.vx[x as usize] = self.dt;

        None
    }

    fn op_fx0a(&mut self, x: u8) -> Option<u8> {
        // LD x, KP
        let key_pos = self.kp.iter().position(|&x| x == true);
        if key_pos.is_some() {
            self.vx[x as usize] = key_pos.unwrap() as u8;

            None
        } else {
            // returning 2 is understood as a call to wait, so the main() loop
            // knows to halt the control flow and wait for another key press

            Some(2)
        }
    }

    fn op_fx15(&mut self, x: u8) -> Option<u8> {
        // LD DT, x
        self.dt = self.vx[x as usize];

        None
    }

    fn op_fx18(&mut self, x: u8) -> Option<u8> {
        // LD ST, x
        self.st = self.vx[x as usize];

        None
    }

    fn op_fx1e(&mut self, x: u8) -> Option<u8> {
        // ADD I, x
        self.ir += self.vx[x as usize] as u16;

        None
    }

    fn op_fx29(&mut self, x: u8) -> Option<u8> {
        // LD F, x
        self.ir = (self.vx[x as usize] as u16) * 5;

        None
    }

    fn op_fx33(&mut self, x: u8) -> Option<u8> {
        // LD B, x
        let start_addr = self.ir as usize;
        self.ram[start_addr] = self.vx[x as usize] / 100;
        self.ram[start_addr + 1] = (self.vx[x as usize] % 100) / 10;
        self.ram[start_addr + 2] = self.vx[x as usize] % 10;

        None
    }

    fn op_fx55(&mut self, x: u8) -> Option<u8> {
        // LD [I], x
        let start_addr = self.ir as usize;
        self.ram[start_addr..=(start_addr + (x as usize))]
            .copy_from_slice(&self.vx[0..=(x as usize)]);

        None
    }

    fn op_fx65(&mut self, x: u8) -> Option<u8> {
        // LD x, [I]
        let start_addr = self.ir as usize;
        self.vx[0..=(x as usize)]
            .copy_from_slice(&self.ram[start_addr..=(start_addr + (x as usize))]);

        None
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fonts() {
        let memory = Cpu::init();

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
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];

        assert_eq!(memory.ram[0x050..0x0A0], exp_result);
    }

    #[test]
    fn test_display() {
        let mut display = Cpu::init();

        let screen_copy: Vec<bool> = vec![false; 2048];

        display.screen[0..32].fill(true);

        let mut exp_result: Vec<bool> = vec![false; 2048];
        exp_result[0..32].fill(true);

        let result: Vec<bool> = display
            .screen
            .iter()
            .zip(screen_copy.iter())
            .map(|(&x1, &x2)| x1 ^ x2)
            .collect();

        assert_eq!(exp_result, result);
    }

    #[test]
    fn test_00e0() {
        // CLS
        let mut cpu = Cpu::init();
        cpu.screen = vec![true; 64 * 32];

        cpu.ram[0x200] = 0x00;
        cpu.ram[0x201] = 0xe0;

        let opcode = cpu.fetch();
        let ins = Instruction::decode(opcode).unwrap();
        let _ = cpu.execute(ins);

        assert_eq!(cpu.screen, vec![false; 64 * 32]);
    }

    #[test]
    fn test_00ee() {
        // RET
        for stack_idx in 1..=15 {
            let mut cpu = Cpu::init();
            let ret_addr: u16 = 0x250;
            cpu.sp = stack_idx;
            cpu.stack[cpu.sp as usize] = ret_addr;

            cpu.ram[0x200] = 0x00;
            cpu.ram[0x201] = 0xee;

            let opcode = cpu.fetch();
            let ins = Instruction::decode(opcode).unwrap();
            let _ = cpu.execute(ins);

            assert_eq!(cpu.pc, ret_addr);
            assert_eq!(cpu.sp, stack_idx - 1);
        }
    }

    #[test]
    fn test_1nnn() {
        // JP addr
        let mut cpu = Cpu::init();
        let jp_addr: u16 = 0x250;

        let tmp = 0x1000 | jp_addr;
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let opcode = cpu.fetch();
        let ins = Instruction::decode(opcode).unwrap();
        let _ = cpu.execute(ins);

        assert_eq!(cpu.pc, jp_addr);
    }

    #[test]
    fn test_2nnn() {
        // CALL addr
        for stack_idx in 0..=14 {
            let mut cpu = Cpu::init();
            let sr_addr: u16 = 0x250;
            cpu.sp = stack_idx;

            let tmp = 0x2000 | sr_addr;
            cpu.ram[0x200] = (tmp >> 8) as u8;
            cpu.ram[0x201] = tmp as u8;

            let opcode = cpu.fetch();
            let ins = Instruction::decode(opcode).unwrap();
            let _ = cpu.execute(ins);

            assert_eq!(cpu.stack[cpu.sp as usize], 0x202);
            assert_eq!(cpu.pc, 0x250);
            assert_eq!(cpu.sp, stack_idx + 1);
        }
    }

    #[test]
    fn test_3xkk() {
        // SE x, kk
        let mut cpu = Cpu::init();
        let kk: u8 = 0x25;
        let x: u8 = 0x1;
        cpu.vx[x as usize] = kk;

        let tmp: u16 = 0x3000 | ((x as u16) << 8) | (kk as u16);
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.execute(ins);

        assert_eq!(cpu.pc, 0x204);
    }

    #[test]
    fn test_4xkk() {
        // SNE x, kk
        let mut cpu = Cpu::init();
        let kk: u8 = 0x24;
        let x: u8 = 0x1;
        cpu.vx[x as usize] = kk + 1;

        let tmp: u16 = 0x4000 | ((x as u16) << 8) | (kk as u16);
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.execute(ins);

        assert_eq!(cpu.pc, 0x204);
    }

    #[test]
    fn test_5xy0() {
        // SE x, y
        let mut cpu = Cpu::init();
        let x: u8 = 0x1;
        let y: u8 = 0xA;
        let val: u8 = 0x25;
        cpu.vx[x as usize] = val;
        cpu.vx[y as usize] = val;

        let tmp: u16 = 0x5000 | ((x as u16) << 8) | ((y as u16) << 4);
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.execute(ins);

        assert_eq!(cpu.pc, 0x204);
    }

    #[test]
    fn test_6xkk() {
        // LD x, kk
        let mut cpu = Cpu::init();
        let x: u8 = 0x1;
        let kk: u8 = 0x25;

        let tmp: u16 = 0x6000 | ((x as u16) << 8) | (kk as u16);
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.execute(ins);

        assert_eq!(cpu.vx[x as usize], kk);
    }

    #[test]
    fn test_7xkk() {
        // ADD x, kk
        let mut cpu = Cpu::init();
        let x: u8 = 0x1;
        let kk: u8 = 0x25;
        let orig: u8 = 0x40;
        cpu.vx[x as usize] = orig;

        let tmp = 0x7000 | ((x as u16) << 8) | (kk as u16);
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.execute(ins);

        assert_eq!(cpu.vx[x as usize], orig + kk);
    }

    #[test]
    fn test_8xy0() {
        // LD x, y
        let mut cpu = Cpu::init();
        let x: u8 = 0x1;
        let y: u8 = 0xA;
        cpu.vx[y as usize] = 0x25;

        let tmp = 0x8000 | ((x as u16) << 8) | ((y as u16) << 4);
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.execute(ins);

        assert_eq!(cpu.vx[x as usize], cpu.vx[y as usize]);
    }

    #[test]
    fn test_8xy1() {
        // OR x, y
        let mut cpu = Cpu::init();
        let x: u8 = 0x1;
        let y: u8 = 0xA;
        let x_val: u8 = 0x25;
        let y_val: u8 = 0x80;
        cpu.vx[x as usize] = x_val;
        cpu.vx[y as usize] = y_val;

        let tmp = 0x8000 | ((x as u16) << 8) | ((y as u16) << 4) | 0x1;
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.execute(ins);

        assert_eq!(cpu.vx[x as usize], x_val | y_val);
    }

    #[test]
    fn test_8xy2() {
        // AND x, y
        let mut cpu = Cpu::init();
        let x: u8 = 0x1;
        let y: u8 = 0xA;
        let x_val: u8 = 0x25;
        let y_val: u8 = 0x80;
        cpu.vx[x as usize] = x_val;
        cpu.vx[y as usize] = y_val;

        let tmp = 0x8000 | ((x as u16) << 8) | ((y as u16) << 4) | 0x2;
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.execute(ins);

        assert_eq!(cpu.vx[x as usize], x_val & y_val);
    }

    #[test]
    fn test_8xy3() {
        // XOR x, y
        let mut cpu = Cpu::init();
        let x: u8 = 0x1;
        let y: u8 = 0xA;
        let x_val: u8 = 0x25;
        let y_val: u8 = 0x80;
        cpu.vx[x as usize] = x_val;
        cpu.vx[y as usize] = y_val;

        let tmp = 0x8000 | ((x as u16) << 8) | ((y as u16) << 4) | 0x3;
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.execute(ins);

        assert_eq!(cpu.vx[x as usize], x_val ^ y_val);
    }

    #[test]
    fn test_8xy4() {
        // ADD x, y
        let mut cpu = Cpu::init();
        let x: u8 = 0x1;
        let y: u8 = 0xA;
        let x_val: u8 = 0x01;
        let y_val: u8 = 0xFF;
        cpu.vx[x as usize] = x_val;
        cpu.vx[y as usize] = y_val;
        let sum: u16 = (x_val as u16) + (y_val as u16);

        let tmp = 0x8000 | ((x as u16) << 8) | ((y as u16) << 4) | 0x4;
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.execute(ins);

        assert_eq!(cpu.vx[x as usize], sum as u8);
        assert_eq!(cpu.vx[0xF] == 1, (0xFF00 & sum) > 1);
    }

    #[test]
    fn test_8xy5() {
        // SUB x, y
        let mut cpu = Cpu::init();
        let x: u8 = 0x1;
        let y: u8 = 0xA;
        let x_val: u8 = 0x05;
        let y_val: u8 = 0xC0;
        cpu.vx[x as usize] = x_val;
        cpu.vx[y as usize] = y_val;
        let diff: u16 = if x_val > y_val {
            (x_val as u16) - (y_val as u16)
        } else {
            (y_val as u16) - (x_val as u16)
        };

        let tmp = 0x8000 | ((x as u16) << 8) | ((y as u16) << 4) | 0x5;
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.execute(ins);

        assert_eq!(cpu.vx[x as usize], diff as u8);
        assert_eq!(cpu.vx[0xF] == 1, x_val > y_val);
    }

    #[test]
    fn test_8xy6() {
        // SHR x {, y}
        let mut cpu = Cpu::init();
        let x: u8 = 0x1;
        let y: u8 = 0xA;
        let x_val: u8 = 0x4;
        cpu.vx[x as usize] = x_val;

        let tmp = 0x8000 | ((x as u16) << 8) | ((y as u16) << 4) | 0x6;
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.execute(ins);

        assert_eq!(cpu.vx[x as usize], x_val >> 1);
        assert_eq!(cpu.vx[0xF] == 1, x_val.trailing_ones() > 1);
    }

    #[test]
    fn test_8xy7() {
        // SUBN x, y
        let mut cpu = Cpu::init();
        let x: u8 = 0x1;
        let y: u8 = 0xA;
        let x_val: u8 = 0xFF;
        let y_val: u8 = 0x01;
        cpu.vx[x as usize] = x_val;
        cpu.vx[y as usize] = y_val;
        let diff: u16 = if y_val > x_val {
            (y_val as u16) - (x_val as u16)
        } else {
            (x_val as u16) - (y_val as u16)
        };

        let tmp = 0x8000 | ((x as u16) << 8) | ((y as u16) << 4) | 0x7;
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.execute(ins);

        assert_eq!(cpu.vx[x as usize], diff as u8);
        assert_eq!(cpu.vx[0xF] == 1, y_val > x_val);
    }

    #[test]
    fn test_8xye() {
        // SHL x {, y}
        let mut cpu = Cpu::init();
        let x: u8 = 0x1;
        let y: u8 = 0xA;
        let x_val: u8 = 0x4;
        cpu.vx[x as usize] = x_val;

        let tmp = 0x8000 | ((x as u16) << 8) | ((y as u16) << 4) | 0xE;
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.execute(ins);

        assert_eq!(cpu.vx[x as usize], x_val << 1);
        assert_eq!(cpu.vx[0xF] == 1, x_val.leading_ones() > 1);
    }

    #[test]
    fn test_9xy0() {
        // SNE x, y
        let mut cpu = Cpu::init();
        let x: u8 = 0x1;
        let y: u8 = 0xA;
        let val: u8 = 0x25;
        cpu.vx[x as usize] = val;
        cpu.vx[y as usize] = val + 1;

        let tmp: u16 = 0x9000 | ((x as u16) << 8) | ((y as u16) << 4);
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.execute(ins);

        assert_eq!(cpu.pc, 0x204);
    }

    #[test]
    fn test_annn() {
        // LD I, addr
        let mut cpu = Cpu::init();
        let addr: u16 = 0x250;

        let tmp: u16 = 0xA000 | addr;
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.execute(ins);

        assert_eq!(cpu.ir, addr);
    }

    #[test]
    fn test_bnnn() {
        // JP V0, addr
        let mut cpu = Cpu::init();
        cpu.vx[0x0] = 0x25;
        let addr: u16 = 0x250;

        let tmp: u16 = 0xB000 | addr;
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.execute(ins);

        assert_eq!(cpu.pc, (cpu.vx[0x0] as u16) + addr);
    }

    #[test]
    fn test_cxkk() {
        // RND x, kk
        let mut cpu = Cpu::init();
        let x: u8 = 0xF;
        let kk: u8 = 0xFF;
        let val: u8 = 0xF;
        cpu.vx[x as usize] = val;

        let tmp: u16 = 0xC000 | (x as u16) << 8 | (kk as u16);
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.execute(ins);

        assert_eq!(cpu.vx[x as usize], 0);
    }

    #[test]
    fn test_dxyn() {
        // DRW x, y, n
        let mut cpu = Cpu::init();
        let x: u8 = 0x1;
        let y: u8 = 0xA;
        cpu.vx[x as usize] = 0x5;
        cpu.vx[y as usize] = 0x7;
        cpu.ir = 0x250;
        let n: u8 = 2;

        let tmp: u16 = 0xD000 | ((x as u16) << 8) | ((y as u16) << 4) | (n as u16);
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;
        for i in 0..n {
            cpu.ram[(cpu.ir + i as u16) as usize] = i;
        }
        cpu.screen[0x20C] = true;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.execute(ins);

        assert_eq!(cpu.vx[0xF], 1);
        assert_eq!(cpu.screen[0x20C], false);
    }

    #[test]
    fn test_ex9e() {
        // SKP x
        let mut cpu = Cpu::init();
        let x: u8 = 0x1;
        let key: u8 = 0x5;
        cpu.vx[x as usize] = key;
        cpu.kp[key as usize] = true;

        let tmp: u16 = 0xE000 | ((x as u16) << 8) | 0x9E;
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.execute(ins);

        assert_eq!(cpu.pc, 0x204);
    }

    #[test]
    fn test_exa1() {
        // SKNP x
        let mut cpu = Cpu::init();
        let x: u8 = 0x1;
        let key: u8 = 0x5;
        cpu.vx[x as usize] = key;
        cpu.kp[(key - 1) as usize] = true;

        let tmp: u16 = 0xE000 | ((x as u16) << 8) | 0xA1;
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.execute(ins);

        assert_eq!(cpu.pc, 0x204);
    }

    #[test]
    fn test_fx07() {
        // LD x, dt
        let mut cpu = Cpu::init();
        let x: u8 = 0x1;
        cpu.dt = 0x25;

        let tmp: u16 = 0xF000 | ((x as u16) << 8) | 0x7;
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.execute(ins);

        assert_eq!(cpu.vx[x as usize], cpu.dt);
    }

    #[test]
    fn test_fx0a() {
        // LD x, kp
        let mut cpu = Cpu::init();
        let x: u8 = 0x1;
        let key: u8 = 0x5;
        cpu.kp[key as usize] = true;

        let tmp: u16 = 0xF000 | ((x as u16) << 8) | 0xA;
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.execute(ins);

        assert_eq!(cpu.vx[x as usize], key);
    }

    #[test]
    fn test_fx15() {
        // LD dt, x
        let mut cpu = Cpu::init();
        let x: u8 = 0x1;
        cpu.dt = 0x25;

        let tmp: u16 = 0xF000 | ((x as u16) << 8) | 0x15;
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.execute(ins);

        assert_eq!(cpu.vx[x as usize], cpu.dt);
    }

    #[test]
    fn test_fx18() {
        // LD st, x
        let mut cpu = Cpu::init();
        let x: u8 = 0x1;
        cpu.st = 0x25;

        let tmp: u16 = 0xF000 | ((x as u16) << 8) | 0x18;
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.execute(ins);

        assert_eq!(cpu.vx[x as usize], cpu.st);
    }

    #[test]
    fn test_fx1e() {
        // ADD ir, x
        let mut cpu = Cpu::init();
        let x: u8 = 0x1;
        let initial_ir: u16 = 0xF01E;
        cpu.ir = initial_ir;
        cpu.vx[x as usize] = 0x25;

        let tmp: u16 = 0xF000 | ((x as u16) << 8) | 0x1E;
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.execute(ins);

        assert_eq!(cpu.ir, (cpu.vx[x as usize] as u16) + initial_ir);
    }

    #[test]
    fn test_fx29() {
        // LD F, vx
        let mut cpu = Cpu::init();
        let x: u8 = 0x1;
        cpu.vx[x as usize] = 0x25;

        let tmp: u16 = 0xF000 | ((x as u16) << 8) | 0x29;
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.execute(ins);

        assert_eq!(cpu.ir, (cpu.vx[x as usize] as u16) * 5);
    }

    #[test]
    fn test_fx33() {
        // LD B, x
        let mut cpu = Cpu::init();
        let x: u8 = 0x1;
        cpu.vx[x as usize] = 137;
        cpu.ir = 0x250;

        let tmp: u16 = 0xF000 | ((x as u16) << 8) | 0x33;
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.execute(ins);

        assert_eq!(cpu.ram[cpu.ir as usize], 1);
        assert_eq!(cpu.ram[(cpu.ir + 1) as usize], 3);
        assert_eq!(cpu.ram[(cpu.ir + 2) as usize], 7);
    }

    #[test]
    fn test_fx55() {
        // LD [ir], x
        let mut cpu = Cpu::init();
        let x: u8 = 0x5;
        for i in 0..x {
            cpu.vx[i as usize] = i * 16;
        }

        let tmp: u16 = 0xF000 | ((x as u16) << 8) | 0x55;
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.execute(ins);

        assert_eq!(
            cpu.ram[(cpu.ir as usize)..=((cpu.ir + (x as u16)) as usize)],
            cpu.vx[0..=(x as usize)]
        );
    }

    #[test]
    fn test_fx65() {
        // LD x, [ir]
        let mut cpu = Cpu::init();
        let x: u8 = 0x5;
        cpu.ir = 0x250;
        for i in 0..x {
            cpu.ram[(cpu.ir as usize) + (i as usize)] = i * 16;
        }

        let tmp: u16 = 0xF000 | ((x as u16) << 8) | 0x65;
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.execute(ins);

        assert_eq!(
            cpu.ram[(cpu.ir as usize)..=((cpu.ir + (x as u16)) as usize)],
            cpu.vx[0..=(x as usize)]
        );
    }
}
