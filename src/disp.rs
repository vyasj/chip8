pub struct Display {
	pub height: u8,
	pub width: u8,
	pub screen: Vec<u8>
}

impl Display {
	pub fn init() -> Display {
		let ret = Display {
			height: 32,
			width: 64,
			screen: vec![0; 64*32]
		};

		ret
	}

	pub fn update(&mut self) {
		let mut idx: usize = 0;
		while self.screen[idx] == 0 {
			idx = idx + 1;

			if idx >= self.screen.len() {
				idx = 0;
				self.screen[idx] = 1;
			}
		}
		self.screen[idx] = 0;
		self.screen[idx+1] = 1;
	}

	pub fn draw(&self, frame: &mut [u8]) {
		for (idx, pixel) in frame.chunks_exact_mut(4).enumerate() {
			let the_shit = self.screen[idx] == 1;

			let rgba = if the_shit {
				[0x00, 0x00, 0x00, 0xFF]
			} else {
				[0xFF, 0xFF, 0xFF, 0xFF]
			};

			pixel.copy_from_slice(&rgba);
		}
	}
}