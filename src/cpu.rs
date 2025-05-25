use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

use rand::Rng;

pub const SCREEN_WIDTH: u8 = 64;
pub const SCREEN_HEIGHT: u8 = 32;

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

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RAW0 => write!(f, "RAW0"),
            Self::CLS => write!(f, "CLS"),
            Self::RET => write!(f, "RET"),
            Self::SYS(nnn) => write!(f, "SYS {:#x}", nnn),
            Self::JP(JPType::Addr(nnn)) => write!(f, "JP {:#x}", nnn),
            Self::JP(JPType::FromV0(nnn)) => write!(f, "JP V0 + {:#x}", nnn),
            Self::CALL(nnn) => write!(f, "CALL {:#x}", nnn),
            Self::SE(x, SEType::Byte(kk)) => write!(f, "SE V{:#x}, {:#x}", x, kk),
            Self::SE(x, SEType::Reg(y)) => write!(f, "SE V{:#x}, V{:#x}", x, y),
            Self::SNE(x, SEType::Byte(kk)) => write!(f, "SNE V{:#x}, {:#x}", x, kk),
            Self::SNE(x, SEType::Reg(y)) => write!(f, "SNE V{:#x}, V{:#x}", x, y),
            Self::LD(_, LDType::Addr(nnn)) => write!(f, "LD I, {:#x}", nnn),
            Self::LD(x, LDType::B) => write!(f, "LD B, V{:#x}", x),
            Self::LD(x, LDType::Byte(kk)) => write!(f, "LD V{:#x}, {:#x}", x, kk),
            Self::LD(x, LDType::F) => write!(f, "LD F, V{:#x}", x),
            Self::LD(x, LDType::FromDT) => write!(f, "LD V{:#x}, DT", x),
            Self::LD(x, LDType::FromI) => write!(f, "LD V{:#x}, [I]", x),
            Self::LD(x, LDType::KeyPress) => write!(f, "LD V{:#x}, K", x),
            Self::LD(x, LDType::Reg(y)) => write!(f, "LD V{:#x}, V{:#x}", x, y),
            Self::LD(x, LDType::ToDT) => write!(f, "LD DT, V{:#x}", x),
            Self::LD(x, LDType::ToI) => write!(f, "LD [I], V{:#x}", x),
            Self::LD(x, LDType::ToST) => write!(f, "LD ST, V{:#x}", x),
            Self::ADD(x, AddType::Byte(kk)) => write!(f, "ADD V{:#x}, {:#x}", x, kk),
            Self::ADD(x, AddType::I) => write!(f, "ADD V{:#x}, I", x),
            Self::ADD(x, AddType::Reg(y)) => write!(f, "ADD V{:#x}, V{:#x}", x, y),
            Self::OR(x, y) => write!(f, "OR V{:#x}, V{:#x}", x, y),
            Self::AND(x, y) => write!(f, "AND V{:#x}, V{:#x}", x, y),
            Self::XOR(x, y) => write!(f, "XOR V{:#x}, V{:#x}", x, y),
            Self::SUB(x, y) => write!(f, "SUB V{:#x}, V{:#x}", x, y),
            Self::SHR(x, y) => write!(f, "SHR V{:#x} {{, V{:#x}}}", x, y),
            Self::SUBN(x, y) => write!(f, "SUBN V{:#x}, V{:#x}", x, y),
            Self::SHL(x, y) => write!(f, "SHL V{:#x} {{, V{:#x}}}", x, y),
            Self::RND(x, kk) => write!(f, "RND V{:#x}, {:#x}", x, kk),
            Self::DRW(x, y, n) => write!(f, "DRW V{:#x}, V{:#x}, {:#x}", x, y, n),
            Self::SKP(x) => write!(f, "SKP V{:#x}", x),
            Self::SKNP(x) => write!(f, "SKNP V{:#x}", x),
        }
    }
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
}

pub struct Cpu {
    /// Memory / 4KB addressable RAM
    pub ram: [u8; 4096],
    /// Memory / 16-level call stack
    pub stack: [u16; 16],
    /// Register / V0–VF general-purpose 8-bit registers
    pub vx: [u8; 16],
    /// Register / Program counter
    pub pc: u16,
    /// Register / Index register
    pub ir: u16,
    /// Register / Stack pointer
    pub sp: u8,
    /// Timer / Delay timer
    pub dt: u8,
    /// Timer / Sound timer
    pub st: u8,
    /// Input / 16-key keypad state
    pub kp: [bool; 16],
    /// Monochrome pixel buffer
    pub screen: [bool; SCREEN_HEIGHT as usize * SCREEN_WIDTH as usize],
    /// Flag / Set when display needs redraw
    pub should_draw: bool,
    /// Flag / Set if waiting for key input
    pub waiting_for_key: bool,
}

impl Cpu {
    pub fn new() -> Self {
        // Memory
        let mut ram = [0x00; 4096];
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
        ram[0x050..0x0A0].copy_from_slice(&font);

        // Stack
        let stack = [0x0000; 16];

        // Registers
        let vx = [0x00; 16];
        let pc = 0x200;
        let ir = 0x00;
        let sp = 0x00;

        // Timers
        let dt = 0x3c;
        let st = 0x3c;

        // Input
        let kp = [false; 16];

        // Display
        let screen = [false; SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize];

        // Flags
        let should_draw = false;
        let waiting_for_key = false;

        Cpu {
            ram,
            stack,
            pc,
            ir,
            vx,
            st,
            dt,
            sp,
            kp,
            screen,
            should_draw,
            waiting_for_key,
        }
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
                println!("{}", ins.unwrap());
            }
        }

        self.pc = start_pc;
    }

    pub fn dump_state(&self) {
        println!("\nregisters:");
        for i in 0..self.vx.len() {
            println!("V{} | {:#x} | ", i, self.vx[i]);
        }

        println!("\nstack:");
        for i in 0..self.stack.len() {
            println!("V{} | {:#x} | ", i, self.stack[i]);
        }

        println!("\nprogram counter: {:#x}", self.pc);
        println!("\nindex register: {:#x}", self.ir);

        println!("\nkeypad:");
        for i in 0..self.kp.len() {
            println!("V{} | {} | ", i, self.kp[i]);
        }
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

    pub fn exec(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ADD(x, AddType::Byte(kk)) => self.exec_add_byte(x, kk),
            Instruction::ADD(x, AddType::I) => self.exec_add_i(x),
            Instruction::ADD(x, AddType::Reg(y)) => self.exec_add_reg(x, y),
            Instruction::AND(x, y) => self.exec_and(x, y),
            Instruction::CALL(nnn) => self.exec_call(nnn),
            Instruction::CLS => self.exec_cls(),
            Instruction::DRW(x, y, n) => self.exec_drw(x, y, n),
            Instruction::JP(JPType::Addr(nnn)) => self.exec_jp(nnn),
            Instruction::JP(JPType::FromV0(nnn)) => self.exec_jp_from0(nnn),
            Instruction::LD(_, LDType::Addr(nnn)) => self.exec_ld_addr(nnn),
            Instruction::LD(x, LDType::B) => self.exec_ld_b(x),
            Instruction::LD(x, LDType::Byte(kk)) => self.exec_ld_byte(x, kk),
            Instruction::LD(x, LDType::F) => self.exec_ld_f(x),
            Instruction::LD(x, LDType::FromDT) => self.exec_ld_from_dt(x),
            Instruction::LD(x, LDType::FromI) => self.exec_ld_from_i(x),
            Instruction::LD(x, LDType::KeyPress) => self.exec_ld_from_kp(x),
            Instruction::LD(x, LDType::Reg(y)) => self.exec_ld_reg(x, y),
            Instruction::LD(x, LDType::ToDT) => self.exec_ld_to_dt(x),
            Instruction::LD(x, LDType::ToI) => self.exec_ld_to_i(x),
            Instruction::LD(x, LDType::ToST) => self.exec_ld_to_st(x),
            Instruction::OR(x, y) => self.exec_or(x, y),
            Instruction::RAW0 => (),
            Instruction::RET => self.exec_ret(),
            Instruction::RND(x, kk) => self.exec_rnd(x, kk),
            Instruction::SE(x, SEType::Byte(kk)) => self.exec_se_byte(x, kk),
            Instruction::SE(x, SEType::Reg(y)) => self.exec_se_reg(x, y),
            Instruction::SHL(x, y) => self.exec_shl(x, y),
            Instruction::SHR(x, y) => self.exec_shr(x, y),
            Instruction::SKNP(x) => self.exec_sknp(x),
            Instruction::SKP(x) => self.exec_skp(x),
            Instruction::SNE(x, SEType::Byte(kk)) => self.exec_sne_byte(x, kk),
            Instruction::SNE(x, SEType::Reg(y)) => self.exec_sne_reg(x, y),
            Instruction::SUB(x, y) => self.exec_sub(x, y),
            Instruction::SUBN(x, y) => self.exec_subn(x, y),
            Instruction::SYS(nnn) => self.exec_sys(nnn),
            Instruction::XOR(x, y) => self.exec_xor(x, y),
        }
    }

    #[inline]
    fn exec_add_byte(&mut self, x: u8, kk: u8) {
        self.vx[x as usize] = self.vx[x as usize].wrapping_add(kk);
    }

    #[inline]
    fn exec_add_i(&mut self, x: u8) {
        self.ir += self.vx[x as usize] as u16;
    }

    #[inline]
    fn exec_add_reg(&mut self, x: u8, y: u8) {
        let sum: u16 = (self.vx[x as usize] as u16) + (self.vx[y as usize] as u16);
        if sum & 0xFF00 > 0 {
            self.vx[0xF] = 1;
        }
        self.vx[x as usize] = sum as u8;
    }

    #[inline]
    fn exec_and(&mut self, x: u8, y: u8) {
        self.vx[x as usize] &= self.vx[y as usize];
    }

    #[inline]
    fn exec_call(&mut self, nnn: u16) {
        self.sp += 1;
        self.stack[self.sp as usize] = self.pc;
        self.pc = nnn;
    }

    #[inline]
    fn exec_cls(&mut self) {
        for x in &mut self.screen {
            *x = false;
        }
    }

    #[inline]
    fn exec_drw(&mut self, x: u8, y: u8, n: u8) {
        self.vx[0xF] = 0;
        for i in 0..n {
            let sprite_byte: u8 = self.ram[(self.ir + i as u16) as usize];
            for j in 0..8 {
                if (sprite_byte << j) & 0x80 != 0 {
                    let counter_x = ((self.vx[x as usize] + j) % (SCREEN_WIDTH)) as usize;
                    let counter_y = ((self.vx[y as usize] + i) % (SCREEN_HEIGHT)) as usize;
                    let screen_idx = (counter_y * SCREEN_WIDTH as usize) + counter_x;
                    if self.screen[screen_idx] {
                        self.vx[0xF] = 1;
                    }
                    self.screen[screen_idx] ^= true;
                }
            }
        }
        self.should_draw = true;
    }

    #[inline]
    fn exec_jp(&mut self, nnn: u16) {
        self.pc = nnn;
    }

    #[inline]
    fn exec_jp_from0(&mut self, nnn: u16) {
        self.pc = (self.vx[0] as u16) + nnn;
    }

    #[inline]
    fn exec_ld_addr(&mut self, nnn: u16) {
        self.ir = nnn;
    }

    #[inline]
    fn exec_ld_b(&mut self, x: u8) {
        let start_addr = self.ir as usize;
        self.ram[start_addr] = self.vx[x as usize] / 100;
        self.ram[start_addr + 1] = (self.vx[x as usize] % 100) / 10;
        self.ram[start_addr + 2] = self.vx[x as usize] % 10;
    }

    #[inline]
    fn exec_ld_byte(&mut self, x: u8, kk: u8) {
        self.vx[x as usize] = kk;
    }

    #[inline]
    fn exec_ld_f(&mut self, x: u8) {
        self.ir = (self.vx[x as usize] as u16) * 5;
    }

    #[inline]
    fn exec_ld_from_dt(&mut self, x: u8) {
        self.vx[x as usize] = self.dt;
    }

    #[inline]
    fn exec_ld_from_i(&mut self, x: u8) {
        let start_addr = self.ir as usize;
        self.vx[0..=(x as usize)]
            .copy_from_slice(&self.ram[start_addr..=(start_addr + (x as usize))]);
    }

    #[inline]
    fn exec_ld_from_kp(&mut self, x: u8) {
        let key_pos = self.kp.iter().position(|&x| x);
        if let Some(pos) = key_pos {
            self.vx[x as usize] = pos as u8;
            self.waiting_for_key = false;
        } else {
            self.waiting_for_key = true;
        }
    }

    #[inline]
    fn exec_ld_reg(&mut self, x: u8, y: u8) {
        self.vx[x as usize] = self.vx[y as usize];
    }

    #[inline]
    fn exec_ld_to_dt(&mut self, x: u8) {
        self.dt = self.vx[x as usize];
    }

    #[inline]
    fn exec_ld_to_i(&mut self, x: u8) {
        let start_addr = self.ir as usize;
        self.ram[start_addr..=(start_addr + (x as usize))]
            .copy_from_slice(&self.vx[0..=(x as usize)]);
    }

    #[inline]
    fn exec_ld_to_st(&mut self, x: u8) {
        self.st = self.vx[x as usize];
    }

    #[inline]
    fn exec_or(&mut self, x: u8, y: u8) {
        self.vx[x as usize] |= self.vx[y as usize];
    }

    #[inline]
    fn exec_ret(&mut self) {
        self.pc = self.stack[self.sp as usize];
        self.sp -= 1;
    }

    #[inline]
    fn exec_rnd(&mut self, x: u8, kk: u8) {
        let mut rng = rand::rng();
        self.vx[x as usize] = rng.random_range(0..=255) & kk;
    }

    #[inline]
    fn exec_se_byte(&mut self, x: u8, kk: u8) {
        if self.vx[x as usize] == kk {
            self.pc += 2;
        }
    }

    #[inline]
    fn exec_se_reg(&mut self, x: u8, y: u8) {
        if self.vx[x as usize] == self.vx[y as usize] {
            self.pc += 2;
        }
    }

    #[inline]
    fn exec_shl(&mut self, x: u8, _y: u8) {
        self.vx[0xF] = if self.vx[x as usize].leading_ones() > 1 {
            1
        } else {
            0
        };
        self.vx[x as usize] <<= 1;
    }

    #[inline]
    fn exec_shr(&mut self, x: u8, _y: u8) {
        self.vx[0xF] = if self.vx[x as usize].trailing_ones() > 1 {
            1
        } else {
            0
        };
        self.vx[x as usize] >>= 1;
    }

    #[inline]
    fn exec_skp(&mut self, x: u8) {
        let key = self.vx[x as usize] as usize;
        if self.kp[key] {
            self.pc += 2;
        }
    }

    #[inline]
    fn exec_sknp(&mut self, x: u8) {
        let key = self.vx[x as usize] as usize;
        if !self.kp[key] {
            self.pc += 2;
        }
    }

    #[inline]
    fn exec_sne_byte(&mut self, x: u8, kk: u8) {
        if self.vx[x as usize] != kk {
            self.pc += 2;
        }
    }

    #[inline]
    fn exec_sne_reg(&mut self, x: u8, y: u8) {
        if self.vx[x as usize] != self.vx[y as usize] {
            self.pc += 2;
        }
    }

    #[inline]
    fn exec_sub(&mut self, x: u8, y: u8) {
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
    }

    #[inline]
    fn exec_subn(&mut self, x: u8, y: u8) {
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
    }

    #[inline]
    fn exec_sys(&mut self, nnn: u16) {
        println!("Instruction {:#x} is for a system call.", nnn);
    }

    #[inline]
    fn exec_xor(&mut self, x: u8, y: u8) {
        self.vx[x as usize] ^= self.vx[y as usize];
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fonts() {
        let memory = Cpu::new();

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
        let mut display = Cpu::new();

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
        let mut cpu = Cpu::new();
        cpu.screen = [true; 64 * 32];

        cpu.ram[0x200] = 0x00;
        cpu.ram[0x201] = 0xe0;

        let opcode = cpu.fetch();
        let ins = Instruction::decode(opcode).unwrap();
        let _ = cpu.exec(ins);

        assert_eq!(cpu.screen, [false; 64 * 32]);
    }

    #[test]
    fn test_00ee() {
        // RET
        for stack_idx in 1..=15 {
            let mut cpu = Cpu::new();
            let ret_addr: u16 = 0x250;
            cpu.sp = stack_idx;
            cpu.stack[cpu.sp as usize] = ret_addr;

            cpu.ram[0x200] = 0x00;
            cpu.ram[0x201] = 0xee;

            let opcode = cpu.fetch();
            let ins = Instruction::decode(opcode).unwrap();
            let _ = cpu.exec(ins);

            assert_eq!(cpu.pc, ret_addr);
            assert_eq!(cpu.sp, stack_idx - 1);
        }
    }

    #[test]
    fn test_1nnn() {
        // JP addr
        let mut cpu = Cpu::new();
        let jp_addr: u16 = 0x250;

        let tmp = 0x1000 | jp_addr;
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let opcode = cpu.fetch();
        let ins = Instruction::decode(opcode).unwrap();
        let _ = cpu.exec(ins);

        assert_eq!(cpu.pc, jp_addr);
    }

    #[test]
    fn test_2nnn() {
        // CALL addr
        for stack_idx in 0..=14 {
            let mut cpu = Cpu::new();
            let sr_addr: u16 = 0x250;
            cpu.sp = stack_idx;

            let tmp = 0x2000 | sr_addr;
            cpu.ram[0x200] = (tmp >> 8) as u8;
            cpu.ram[0x201] = tmp as u8;

            let opcode = cpu.fetch();
            let ins = Instruction::decode(opcode).unwrap();
            let _ = cpu.exec(ins);

            assert_eq!(cpu.stack[cpu.sp as usize], 0x202);
            assert_eq!(cpu.pc, 0x250);
            assert_eq!(cpu.sp, stack_idx + 1);
        }
    }

    #[test]
    fn test_3xkk() {
        // SE x, kk
        let mut cpu = Cpu::new();
        let kk: u8 = 0x25;
        let x: u8 = 0x1;
        cpu.vx[x as usize] = kk;

        let tmp: u16 = 0x3000 | ((x as u16) << 8) | (kk as u16);
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.exec(ins);

        assert_eq!(cpu.pc, 0x204);
    }

    #[test]
    fn test_4xkk() {
        // SNE x, kk
        let mut cpu = Cpu::new();
        let kk: u8 = 0x24;
        let x: u8 = 0x1;
        cpu.vx[x as usize] = kk + 1;

        let tmp: u16 = 0x4000 | ((x as u16) << 8) | (kk as u16);
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.exec(ins);

        assert_eq!(cpu.pc, 0x204);
    }

    #[test]
    fn test_5xy0() {
        // SE x, y
        let mut cpu = Cpu::new();
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
        let _ = cpu.exec(ins);

        assert_eq!(cpu.pc, 0x204);
    }

    #[test]
    fn test_6xkk() {
        // LD x, kk
        let mut cpu = Cpu::new();
        let x: u8 = 0x1;
        let kk: u8 = 0x25;

        let tmp: u16 = 0x6000 | ((x as u16) << 8) | (kk as u16);
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.exec(ins);

        assert_eq!(cpu.vx[x as usize], kk);
    }

    #[test]
    fn test_7xkk() {
        // ADD x, kk
        let mut cpu = Cpu::new();
        let x: u8 = 0x1;
        let kk: u8 = 0x25;
        let orig: u8 = 0x40;
        cpu.vx[x as usize] = orig;

        let tmp = 0x7000 | ((x as u16) << 8) | (kk as u16);
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.exec(ins);

        assert_eq!(cpu.vx[x as usize], orig + kk);
    }

    #[test]
    fn test_8xy0() {
        // LD x, y
        let mut cpu = Cpu::new();
        let x: u8 = 0x1;
        let y: u8 = 0xA;
        cpu.vx[y as usize] = 0x25;

        let tmp = 0x8000 | ((x as u16) << 8) | ((y as u16) << 4);
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.exec(ins);

        assert_eq!(cpu.vx[x as usize], cpu.vx[y as usize]);
    }

    #[test]
    fn test_8xy1() {
        // OR x, y
        let mut cpu = Cpu::new();
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
        let _ = cpu.exec(ins);

        assert_eq!(cpu.vx[x as usize], x_val | y_val);
    }

    #[test]
    fn test_8xy2() {
        // AND x, y
        let mut cpu = Cpu::new();
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
        let _ = cpu.exec(ins);

        assert_eq!(cpu.vx[x as usize], x_val & y_val);
    }

    #[test]
    fn test_8xy3() {
        // XOR x, y
        let mut cpu = Cpu::new();
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
        let _ = cpu.exec(ins);

        assert_eq!(cpu.vx[x as usize], x_val ^ y_val);
    }

    #[test]
    fn test_8xy4() {
        // ADD x, y
        let mut cpu = Cpu::new();
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
        let _ = cpu.exec(ins);

        assert_eq!(cpu.vx[x as usize], sum as u8);
        assert_eq!(cpu.vx[0xF] == 1, (0xFF00 & sum) > 1);
    }

    #[test]
    fn test_8xy5() {
        // SUB x, y
        let mut cpu = Cpu::new();
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
        let _ = cpu.exec(ins);

        assert_eq!(cpu.vx[x as usize], diff as u8);
        assert_eq!(cpu.vx[0xF] == 1, x_val > y_val);
    }

    #[test]
    fn test_8xy6() {
        // SHR x {, y}
        let mut cpu = Cpu::new();
        let x: u8 = 0x1;
        let y: u8 = 0xA;
        let x_val: u8 = 0x4;
        cpu.vx[x as usize] = x_val;

        let tmp = 0x8000 | ((x as u16) << 8) | ((y as u16) << 4) | 0x6;
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.exec(ins);

        assert_eq!(cpu.vx[x as usize], x_val >> 1);
        assert_eq!(cpu.vx[0xF] == 1, x_val.trailing_ones() > 1);
    }

    #[test]
    fn test_8xy7() {
        // SUBN x, y
        let mut cpu = Cpu::new();
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
        let _ = cpu.exec(ins);

        assert_eq!(cpu.vx[x as usize], diff as u8);
        assert_eq!(cpu.vx[0xF] == 1, y_val > x_val);
    }

    #[test]
    fn test_8xye() {
        // SHL x {, y}
        let mut cpu = Cpu::new();
        let x: u8 = 0x1;
        let y: u8 = 0xA;
        let x_val: u8 = 0x4;
        cpu.vx[x as usize] = x_val;

        let tmp = 0x8000 | ((x as u16) << 8) | ((y as u16) << 4) | 0xE;
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.exec(ins);

        assert_eq!(cpu.vx[x as usize], x_val << 1);
        assert_eq!(cpu.vx[0xF] == 1, x_val.leading_ones() > 1);
    }

    #[test]
    fn test_9xy0() {
        // SNE x, y
        let mut cpu = Cpu::new();
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
        let _ = cpu.exec(ins);

        assert_eq!(cpu.pc, 0x204);
    }

    #[test]
    fn test_annn() {
        // LD I, addr
        let mut cpu = Cpu::new();
        let addr: u16 = 0x250;

        let tmp: u16 = 0xA000 | addr;
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.exec(ins);

        assert_eq!(cpu.ir, addr);
    }

    #[test]
    fn test_bnnn() {
        // JP V0, addr
        let mut cpu = Cpu::new();
        cpu.vx[0x0] = 0x25;
        let addr: u16 = 0x250;

        let tmp: u16 = 0xB000 | addr;
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.exec(ins);

        assert_eq!(cpu.pc, (cpu.vx[0x0] as u16) + addr);
    }

    #[test]
    fn test_cxkk() {
        // RND x, kk
        let mut cpu = Cpu::new();
        let x: u8 = 0xF;
        let kk: u8 = 0xFF;
        let val: u8 = 0xF;
        cpu.vx[x as usize] = val;

        let tmp: u16 = 0xC000 | (x as u16) << 8 | (kk as u16);
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.exec(ins);

        assert_eq!(cpu.vx[x as usize], 0);
    }

    #[test]
    fn test_dxyn() {
        // DRW x, y, n
        let mut cpu = Cpu::new();
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
        let _ = cpu.exec(ins);

        assert_eq!(cpu.vx[0xF], 1);
        assert_eq!(cpu.screen[0x20C], false);
    }

    #[test]
    fn test_ex9e() {
        // SKP x
        let mut cpu = Cpu::new();
        let x: u8 = 0x1;
        let key: u8 = 0x5;
        cpu.vx[x as usize] = key;
        cpu.kp[key as usize] = true;

        let tmp: u16 = 0xE000 | ((x as u16) << 8) | 0x9E;
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.exec(ins);

        assert_eq!(cpu.pc, 0x204);
    }

    #[test]
    fn test_exa1() {
        // SKNP x
        let mut cpu = Cpu::new();
        let x: u8 = 0x1;
        let key: u8 = 0x5;
        cpu.vx[x as usize] = key;
        cpu.kp[(key - 1) as usize] = true;

        let tmp: u16 = 0xE000 | ((x as u16) << 8) | 0xA1;
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.exec(ins);

        assert_eq!(cpu.pc, 0x204);
    }

    #[test]
    fn test_fx07() {
        // LD x, dt
        let mut cpu = Cpu::new();
        let x: u8 = 0x1;
        cpu.dt = 0x25;

        let tmp: u16 = 0xF000 | ((x as u16) << 8) | 0x7;
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.exec(ins);

        assert_eq!(cpu.vx[x as usize], cpu.dt);
    }

    #[test]
    fn test_fx0a() {
        // LD x, kp
        let mut cpu = Cpu::new();
        let x: u8 = 0x1;
        let key: u8 = 0x5;
        cpu.kp[key as usize] = true;

        let tmp: u16 = 0xF000 | ((x as u16) << 8) | 0xA;
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.exec(ins);

        assert_eq!(cpu.vx[x as usize], key);
    }

    #[test]
    fn test_fx15() {
        // LD dt, x
        let mut cpu = Cpu::new();
        let x: u8 = 0x1;
        cpu.dt = 0x25;

        let tmp: u16 = 0xF000 | ((x as u16) << 8) | 0x15;
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.exec(ins);

        assert_eq!(cpu.vx[x as usize], cpu.dt);
    }

    #[test]
    fn test_fx18() {
        // LD st, x
        let mut cpu = Cpu::new();
        let x: u8 = 0x1;
        cpu.st = 0x25;

        let tmp: u16 = 0xF000 | ((x as u16) << 8) | 0x18;
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.exec(ins);

        assert_eq!(cpu.vx[x as usize], cpu.st);
    }

    #[test]
    fn test_fx1e() {
        // ADD ir, x
        let mut cpu = Cpu::new();
        let x: u8 = 0x1;
        let newial_ir: u16 = 0xF01E;
        cpu.ir = newial_ir;
        cpu.vx[x as usize] = 0x25;

        let tmp: u16 = 0xF000 | ((x as u16) << 8) | 0x1E;
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.exec(ins);

        assert_eq!(cpu.ir, (cpu.vx[x as usize] as u16) + newial_ir);
    }

    #[test]
    fn test_fx29() {
        // LD F, vx
        let mut cpu = Cpu::new();
        let x: u8 = 0x1;
        cpu.vx[x as usize] = 0x25;

        let tmp: u16 = 0xF000 | ((x as u16) << 8) | 0x29;
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.exec(ins);

        assert_eq!(cpu.ir, (cpu.vx[x as usize] as u16) * 5);
    }

    #[test]
    fn test_fx33() {
        // LD B, x
        let mut cpu = Cpu::new();
        let x: u8 = 0x1;
        cpu.vx[x as usize] = 137;
        cpu.ir = 0x250;

        let tmp: u16 = 0xF000 | ((x as u16) << 8) | 0x33;
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.exec(ins);

        assert_eq!(cpu.ram[cpu.ir as usize], 1);
        assert_eq!(cpu.ram[(cpu.ir + 1) as usize], 3);
        assert_eq!(cpu.ram[(cpu.ir + 2) as usize], 7);
    }

    #[test]
    fn test_fx55() {
        // LD [ir], x
        let mut cpu = Cpu::new();
        let x: u8 = 0x5;
        for i in 0..x {
            cpu.vx[i as usize] = i * 16;
        }

        let tmp: u16 = 0xF000 | ((x as u16) << 8) | 0x55;
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.exec(ins);

        assert_eq!(
            cpu.ram[(cpu.ir as usize)..=((cpu.ir + (x as u16)) as usize)],
            cpu.vx[0..=(x as usize)]
        );
    }

    #[test]
    fn test_fx65() {
        // LD x, [ir]
        let mut cpu = Cpu::new();
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
        let _ = cpu.exec(ins);

        assert_eq!(
            cpu.ram[(cpu.ir as usize)..=((cpu.ir + (x as u16)) as usize)],
            cpu.vx[0..=(x as usize)]
        );
    }
}
