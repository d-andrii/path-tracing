use std::io::{self};

mod coordinates;
use coordinates::*;

mod colour;
pub use colour::*;

mod bmp;
pub mod formats {
	pub use super::bmp::*;
}

type ImageIndexCapacity = u32;

pub struct Image {
	width: ImageIndexCapacity,
	height: ImageIndexCapacity,
	data: Vec<Colour>,
}

pub trait ImageFormat {
	fn encode(image: &Image) -> io::Result<Vec<u8>>;
}

impl Image {
	pub fn new(width: ImageIndexCapacity, height: ImageIndexCapacity) -> Self {
		Self {
			width,
			height,
			data: vec![Colour::new(); (height * width) as usize],
		}
	}

	pub fn coordinates(&self) -> ImageCoordinates {
		self.into()
	}

	pub fn set_pixel(&mut self, x: ImageIndexCapacity, y: ImageIndexCapacity, colour: Colour) {
		self.data[(x + self.width * y) as usize] = colour;
	}

	pub fn get_pixel(&self, x: ImageIndexCapacity, y: ImageIndexCapacity) -> &Colour {
		&self.data[(x + self.width * y) as usize]
	}
}
