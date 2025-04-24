use std::fs::File;
use std::io::BufReader;
use std::io::Read;

use rand::Rng;

pub enum Opcode {
    Unknown { instruction: u16 },
    Op00e0,
    Op00ee,
    Op0nnn { nnn: u16 },
    Op1nnn { nnn: u16 },
    Op2nnn { nnn: u16 },
    Op3xkk { x: u8, kk: u8},
    Op4xkk { x: u8, kk: u8},
    Op5xy0 { x: u8, y: u8},
    Op6xkk { x: u8, kk: u8},
    Op7xkk { x: u8, kk: u8},
    Op8xy0 { x: u8, y: u8},
    Op8xy1 { x: u8, y: u8},
    Op8xy2 { x: u8, y: u8},
    Op8xy3 { x: u8, y: u8},
    Op8xy4 { x: u8, y: u8},
    Op8xy5 { x: u8, y: u8},
    Op8xy6 { x: u8, y: u8},
    Op8xy7 { x: u8, y: u8},
    Op8xye { x: u8, y: u8},
    Op9xy0 { x: u8, y: u8},
    Opannn { nnn: u16 },
    Opbnnn { nnn: u16 },
    Opcxkk { x: u8, kk: u8},
    Opdxyn { x: u8, y: u8, n: u8},
    Opex9e { x: u8 },
    Opexa1 { x: u8 },
    Opfx07 { x: u8 },
    Opfx0a { x: u8 },
    Opfx15 { x: u8 },
    Opfx18 { x: u8 },
    Opfx1e { x: u8 },
    Opfx29 { x: u8 },
    Opfx33 { x: u8 },
    Opfx55 { x: u8 },
    Opfx65 { x: u8 },
}

pub struct Cpu {
    pub ram: Vec<u8>,
    pub stack: Vec<u16>,

    pub pc: u16,        // program counter
    pub ir: u16,        // index register
    pub vx: Vec<u8>,    // V-registers
    pub st: u8,         // sound timer
    pub dt: u8,         // delay timer
    pub sp: u8,         // stack pointer
    pub kp: Vec<bool>,  // key pressed

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

    pub fn decode(&mut self, instruction: u16) -> Opcode {
        let nibbles = (
            (0xF000 & instruction) >> 12,
            (0x0F00 & instruction) >> 8,
            (0x00F0 & instruction) >> 4,
            0x000F & instruction
        );

        let nnn = 0x0FFF & instruction;
        let kk = (0x00FF & instruction) as u8;
        let n = (0x000F & instruction) as u8;
        let x = (0x0F00 & instruction) as u8;
        let y = (0x00F0 & instruction) as u8;

        match nibbles {
            (0x0, 0x0, 0xE, 0x0) => { Opcode::Op00e0 },
            (0x0, 0x0, 0xE, 0xE) => { Opcode::Op00ee },
            (0x0, _, _, _) =>       { Opcode::Op0nnn { nnn } },
            (0x1, _, _, _) =>       { Opcode::Op1nnn { nnn } },
            (0x2, _, _, _) =>       { Opcode::Op2nnn { nnn } },
            (0x3, _, _, _) =>       { Opcode::Op3xkk { x, kk } },
            (0x4, _, _, _) =>       { Opcode::Op4xkk { x, kk } },
            (0x5, _, _, 0x0) =>     { Opcode::Op5xy0 { x, y } },
            (0x6, _, _, _) =>       { Opcode::Op6xkk { x, kk } },
            (0x7, _, _, _) =>       { Opcode::Op7xkk { x, kk } },
            (0x8, _, _, 0x1) =>     { Opcode::Op8xy1 { x, y } },
            (0x8, _, _, 0x2) =>     { Opcode::Op8xy2 { x, y } },
            (0x8, _, _, 0x3) =>     { Opcode::Op8xy3 { x, y } },
            (0x8, _, _, 0x4) =>     { Opcode::Op8xy4 { x, y } },
            (0x8, _, _, 0x5) =>     { Opcode::Op8xy5 { x, y } },
            (0x8, _, _, 0x6) =>     { Opcode::Op8xy6 { x, y } },
            (0x8, _, _, 0x7) =>     { Opcode::Op8xy7 { x, y } },
            (0x8, _, _, 0xE) =>     { Opcode::Op8xye { x, y } },
            (0x9, _, _, 0x0) =>     { Opcode::Op9xy0 { x, y } },
            (0xA, _, _, _) =>       { Opcode::Opannn { nnn } },
            (0xB, _, _, _) =>       { Opcode::Opbnnn { nnn } },
            (0xC, _, _, _) =>       { Opcode::Opcxkk { x, kk } },
            (0xD, _, _, _) =>       { Opcode::Opdxyn { x, y, n } },
            (0xE, _, 0x9, 0xE) =>   { Opcode::Opex9e { x } },
            (0xE, _, 0xA, 0x1) =>   { Opcode::Opexa1 { x } },
            (0xF, _, 0x0, 0xA) =>   { Opcode::Opfx0a { x } },
            (0xF, _, 0x1, 0x5) =>   { Opcode::Opfx15 { x } },
            (0xF, _, 0x1, 0x8) =>   { Opcode::Opfx18 { x } },
            (0xF, _, 0x1, 0xE) =>   { Opcode::Opfx1e { x } },
            (0xF, _, 0x2, 0x9) =>   { Opcode::Opfx29 { x } },
            (0xF, _, 0x3, 0x3) =>   { Opcode::Opfx33 { x } },
            (0xF, _, 0x5, 0x5) =>   { Opcode::Opfx55 { x } },
            (0xF, _, 0x6, 0x5) =>   { Opcode::Opfx65 { x } },
            (0xF, _, 0x0, 0x7) =>   { Opcode::Opfx07 { x } },
            _ =>                    { Opcode::Unknown { instruction } },
        }
    }

    pub fn execute(&mut self, opcode: Opcode) -> Option<u8> {
        match opcode {
            Opcode::Op00e0 => { self.op_00e0() },
            Opcode::Op00ee => { self.op_00ee() },
            Opcode::Op0nnn { nnn } => { self.op_0nnn(nnn) },
            Opcode::Op1nnn { nnn } => { self.op_1nnn(nnn) },
            Opcode::Op2nnn { nnn} => { self.op_2nnn(nnn) },
            Opcode::Op3xkk { x, kk } => { self.op_3xkk(x, kk) },
            Opcode::Op4xkk { x, kk } => { self.op_4xkk(x, kk) },
            Opcode::Op5xy0 { x, y } => { self.op_5xy0(x, y) },
            Opcode::Op6xkk { x, kk } => { self.op_6xkk(x, kk) },
            Opcode::Op7xkk { x, kk } => { self.op_7xkk(x, kk) },
            Opcode::Op8xy0 { x, y } => { self.op_8xy0(x, y) },
            Opcode::Op8xy1 { x, y } => { self.op_8xy1(x, y) },
            Opcode::Op8xy2 { x, y } => { self.op_8xy2(x, y) },
            Opcode::Op8xy3 { x, y } => { self.op_8xy3(x, y) },
            Opcode::Op8xy4 { x, y } => { self.op_8xy4(x, y) },
            Opcode::Op8xy5 { x, y } => { self.op_8xy5(x, y) },
            Opcode::Op8xy6 { x, y } => { self.op_8xy6(x, y) },
            Opcode::Op8xy7 { x, y } => { self.op_8xy7(x, y) },
            Opcode::Op8xye { x, y } => { self.op_8xye(x, y) },
            Opcode::Op9xy0 { x, y } => { self.op_9xy0(x, y) },
            Opcode::Opannn { nnn } => { self.op_annn(nnn) },
            Opcode::Opbnnn { nnn } => { self.op_bnnn(nnn) },
            Opcode::Opcxkk { x, kk } => { self.op_cxkk(x, kk) },
            Opcode::Opdxyn { x, y, n } => { self.op_dxyn(x, y, n) },
            Opcode::Opex9e { x } => { self.op_ex9e(x) },
            Opcode::Opexa1 { x } => { self.op_exa1(x) },
            Opcode::Opfx07 { x } => { self.op_fx07(x) },
            Opcode::Opfx0a { x } => { self.op_fx0a(x) },
            Opcode::Opfx15 { x } => { self.op_fx15(x) },
            Opcode::Opfx18 { x } => { self.op_fx18(x) },
            Opcode::Opfx1e { x } => { self.op_fx1e(x) },
            Opcode::Opfx29 { x } => { self.op_fx29(x) },
            Opcode::Opfx33 { x } => { self.op_fx33(x) },
            Opcode::Opfx55 { x } => { self.op_fx55(x) },
            Opcode::Opfx65 { x } => { self.op_fx65(x) },
            _ => {
                println!("Unknown opcode");
                None
            }
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
        self.sp -= 1;
        self.pc = self.stack[self.sp as usize];

        None
    }

    fn op_0nnn(&mut self, nnn: u16) -> Option<u8> {
        // SYS addr
        println!("opcode 0{} is for a system call.", nnn);

        None
    }

    fn op_1nnn(&mut self, nnn: u16) -> Option<u8> {
        // JP addr
        self.pc = nnn;

        None
    }

    fn op_2nnn(&mut self, nnn: u16) -> Option<u8> {
        // JSR addr
        self.stack[self.sp as usize] = self.pc;
        self.pc = nnn;
        self.sp += 1;

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
        self.vx[x as usize] += kk;

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
        self.vx[0xF] = if self.vx[x as usize] > self.vx[y as usize] { 1 } else { 0 };
        self.vx[x as usize] = if self.vx[0xF] == 1 { self.vx[x as usize] - self.vx[y as usize] } else { self.vx[y as usize] - self.vx[x as usize] };

        None
    }

    fn op_8xy6(&mut self, x: u8, _y: u8) -> Option<u8> {
        // SHR x {, y}
        if self.vx[x as usize].trailing_ones() > 1 { self.vx[0xF] = 1 } else { self.vx[0xF] = 0 };
        self.vx[x as usize] = self.vx[x as usize] >> 1;

        None
    }

    fn op_8xy7(&mut self, x: u8, y: u8) -> Option<u8> {
        // SUBN x, y
        self.vx[0xF] = if self.vx[y as usize] > self.vx[x as usize] { 1 } else { 0 };
        self.vx[x as usize] = if self.vx[0xF] == 0 { self.vx[x as usize] - self.vx[y as usize] } else { self.vx[y as usize] - self.vx[x as usize] };

        None
    }

    fn op_8xye(&mut self, x: u8, _y: u8) -> Option<u8> {
        // SHR x {, y}
        if self.vx[x as usize].leading_ones() > 1 { self.vx[0xF] = 1 } else { self.vx[0xF] = 0 };
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
        for i in 0..n {
            let sprite_byte: u8 = self.ram[(self.ir + i as u16) as usize];

            for j in 0..8 {
                if (sprite_byte << j) & 0x80 != 0 {
                    let counter_x = (x as usize + j) % (self.width as usize);
                    let counter_y = (y as usize + i as usize) % (self.height as usize);
                    let screen_idx = counter_y * (self.width as usize) + counter_x;

                    if self.screen[screen_idx as usize] {
                        self.vx[0xF] = 1;
                    }
                    self.screen[screen_idx as usize] ^= true;
                }
            }
        }

        // returning 1 is understood as a draw instruction, so the main() loop
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
        let res: Option<u8>;
        if key_pos.is_some() {
            self.vx[x as usize] = key_pos.unwrap() as u8;
            res = None;
        } else {
            // returning 2 is understood as a wait instruction, so the main()
            // loop knows to halt the control flow and wait for another key press
            res = Some(2);
        }

        res
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
        self.ram[start_addr..=(start_addr+(x as usize))].copy_from_slice(&self.vx[0..=(x as usize)]);

        None
    }

    fn op_fx65(&mut self, x: u8) -> Option<u8> {
        // LD x, [I]
        let start_addr = self.ir as usize;
        self.vx[0..=(x as usize)].copy_from_slice(&self.ram[start_addr..=(start_addr+(x as usize))]);

        None
    }
}