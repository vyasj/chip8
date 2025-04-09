use rand::Rng;

pub struct Registers {
    pub pc: u16,        // program counter
    pub ir: u16,        // index register
    pub vx: Vec<u8>,    // V-registers
    pub st: u8,         // sound timer
    pub dt: u8,         // delay timer
    pub sp: u8,         // stack pointer
    pub kp: Vec<bool>,  // key pressed
}

impl Registers {
    pub fn init() -> Registers {
        let ret = Registers {
            pc: 0x200,
            ir: 0,
            vx: vec![0; 16],
            st: 60,
            dt: 60,
            sp: 0,
            kp: vec![false; 16],
        };

        ret
    }

    pub fn gen_random(&self) -> u8 {
        let mut rng = rand::rng();
        let num: u8 = rng.random_range(0..=255);

        num
    }
}
