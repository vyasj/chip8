pub struct Display {
	pub height: u8,
	pub width: u8,
	pub screen: Vec<u16>
}

impl Display {
	pub fn init() -> Display {
		let ret = Display {
			height: 32,
			width: 64,
			screen: vec![0; 32*64]
		};

		ret
	}

	pub fn update(&mut self, vals: Vec<u16>, posx: u8, posy: u8) {
		()
	}

	pub fn render(self) {
		()
	}
}