use std::ops::Add;

use super::{Intersect, Ray, Vec3f};

#[derive(Debug, Clone, Copy)]
pub struct Polygon3 {
	pub a: Vec3f,
	pub b: Vec3f,
	pub c: Vec3f,
	pub normal: Vec3f,
}

impl Polygon3 {
	pub fn new(a: Vec3f, b: Vec3f, c: Vec3f) -> Self {
		Self {
			a,
			b,
			c,
			normal: (a - b).cross(a - c).unit(),
		}
	}

	pub fn scale(self, n: f32) -> Self {
		Self {
			a: self.a * n,
			b: self.b * n,
			c: self.c * n,
			normal: self.normal,
		}
	}
}

impl Add<Vec3f> for Polygon3 {
	type Output = Self;

	fn add(self, other: Vec3f) -> Self {
		Self {
			a: self.a + other,
			b: self.b + other,
			c: self.c + other,
			normal: self.normal,
		}
	}
}

impl Intersect<Ray> for Polygon3 {
	fn intersect(&self, r: &Ray) -> Option<Vec3f> {
		let edge1 = self.b - self.a;
		let edge2 = self.c - self.a;

		let h = r.direction.cross(edge2);
		let a = edge1.dot(h);

		if a > -Self::EPSILON && a < Self::EPSILON {
			return None;
		}

		let f = 1. / a;
		let s = r.origin - self.a;
		let u = f * s.dot(h);
		if u < 0. || u > 1. {
			return None;
		}

		let q = s.cross(edge1);
		let v = f * r.direction.dot(q);
		if v < 0. || (u + v) > 1. {
			return None;
		}

		let t = f * edge2.dot(q);
		if t > Self::EPSILON {
			Some(r.origin + r.direction * t)
		} else {
			None
		}
	}
}
