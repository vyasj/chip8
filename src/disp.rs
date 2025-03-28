pub struct Display {
	pub height: u8,
	pub width: u8,
	pub screen: Vec<bool>
}

impl Display {
	pub fn init() -> Display {
		let ret = Display {
			height: 32,
			width: 64,
			screen: vec![false; 32*64]
		};

		ret
	}
}