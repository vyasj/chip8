use rand::Rng;

pub struct Registers {
    pub pc: u16,
    pub ir: u16,
    pub vx: Vec<u8>,
    pub st: u8,
    pub dt: u8,
    pub sp: u8,
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
        };

        ret
    }

    pub fn gen_random(self) -> u8 {
        let mut rng = rand::rng();
        let num: u8 = rng.random_range(0..=255);

        num
    }
}
