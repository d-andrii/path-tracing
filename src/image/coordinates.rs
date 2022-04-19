pub struct ImageCoordinates {
	width: super::ImageIndexCapacity,
	height: super::ImageIndexCapacity,
	x: super::ImageIndexCapacity,
	y: super::ImageIndexCapacity,
}

impl ImageCoordinates {
	pub fn new(width: super::ImageIndexCapacity, height: super::ImageIndexCapacity) -> Self {
		Self {
			width,
			height,
			x: 0,
			y: 0,
		}
	}
}

impl From<&super::Image> for ImageCoordinates {
	fn from(image: &super::Image) -> Self {
		ImageCoordinates::new(image.width, image.height)
	}
}

impl Iterator for ImageCoordinates {
	type Item = (super::ImageIndexCapacity, super::ImageIndexCapacity);

	fn next(&mut self) -> Option<(super::ImageIndexCapacity, super::ImageIndexCapacity)> {
		if self.x < self.width && self.y < self.height {
			let i = Some((self.x, self.y));

			self.x += 1;
			if self.x == self.width {
				self.x = 0;
				self.y += 1;
			}

			i
		} else {
			None
		}
	}
}
