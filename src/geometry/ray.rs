use super::Vec3f;

#[derive(Debug)]
pub struct Ray {
	pub origin: Vec3f,
	pub direction: Vec3f,
}

impl Ray {
	pub fn new(origin: Vec3f, direction: Vec3f) -> Self {
		Self { origin, direction }
	}
}
