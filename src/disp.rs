pub struct Display {
    pub height: u8,
    pub width: u8,
    pub screen: Vec<bool>,
}

impl Display {
    pub fn init() -> Display {
        let ret = Display {
            height: 32,
            width: 64,
            screen: vec![false; 64 * 32],
        };

        ret
    }

    pub fn update(
        &mut self,
        x: u8,
        y: u8,
        n: u16,
        vx: &mut Vec<u8>,
        ir: u16,
        ram: &Vec<u8>,
    ) {
    	let mut counter_x = x;
    	let mut counter_y = y;

        for i in 0..n {
            let sprite_byte: u8 = ram[(ir + i) as usize];

            for j in 0..8 {
                if (sprite_byte << j) & 0x80 > 0 {
                    let screen_idx: usize = ((counter_y as usize) * (self.width as usize)) + (counter_x as usize);
                    if self.screen[screen_idx] == true {
                        vx[0xF] = 1;
                    } else {
                        self.screen[screen_idx] = true;
                    }

                    if counter_x % 64 == 0 {
                    	counter_x = x;
                        break;
                    } else {
                        counter_x = counter_x + 1;
                    }
                }
            }

            counter_y = counter_y + 1;

            if counter_y % 32 == 0 {
            	counter_y = y;
                break;
            }
        }
    }

    pub fn draw(&self, frame: &mut [u8]) {
        for (idx, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let pixel_on = self.screen[idx];

            let rgba = if pixel_on {
                [0x00, 0x00, 0x00, 0xFF]
            } else {
                [0xFF, 0xFF, 0xFF, 0xFF]
            };

            pixel.copy_from_slice(&rgba);
        }
    }
}
