use std::ops::{Add, Div, Mul, Sub};

use num::Float;
use rand::{distributions::uniform::SampleUniform, Rng};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3<T> {
	pub x: T,
	pub y: T,
	pub z: T,
}

impl<T> Vec3<T>
where
	T: Float + SampleUniform,
{
	pub fn new(x: T, y: T, z: T) -> Self {
		Self { x, y, z }
	}

	pub fn len_sq(&self) -> T {
		self.x * self.x + self.y * self.y + self.z * self.z
	}

	pub fn len(&self) -> T {
		self.len_sq().sqrt()
	}

	pub fn unit(self) -> Self {
		let l = self.len();
		Self {
			x: self.x / l,
			y: self.y / l,
			z: self.z / l,
		}
	}

	pub fn rand(min: T, max: T) -> Self {
		let mut rng = rand::thread_rng();
		Self::new(
			rng.gen_range(min..max),
			rng.gen_range(min..max),
			rng.gen_range(min..max),
		)
	}

	pub fn rand_in_unit() -> Self {
		loop {
			let v = Self::rand(-T::one(), T::one());
			if v.len_sq() < T::one() {
				return v;
			}
		}
	}

	pub fn reflect(self, n: Vec3<T>) -> Self {
		self - (n * self.dot(n)) * (T::one() + T::one())
	}
}

impl<T> Add for Vec3<T>
where
	T: Add<Output = T>,
{
	type Output = Self;

	fn add(self, other: Self) -> Self {
		Self {
			x: self.x + other.x,
			y: self.y + other.y,
			z: self.z + other.z,
		}
	}
}

impl<T> Sub for Vec3<T>
where
	T: Sub<Output = T>,
{
	type Output = Self;

	fn sub(self, other: Self) -> Self {
		Self {
			x: self.x - other.x,
			y: self.y - other.y,
			z: self.z - other.z,
		}
	}
}

impl<T> Mul<T> for Vec3<T>
where
	T: Mul<Output = T> + Copy,
{
	type Output = Self;

	fn mul(self, other: T) -> Self {
		Self {
			x: self.x * other,
			y: self.y * other,
			z: self.z * other,
		}
	}
}

impl<T> Div<T> for Vec3<T>
where
	T: Div<Output = T> + Copy,
{
	type Output = Self;

	fn div(self, other: T) -> Self {
		Self {
			x: self.x / other,
			y: self.y / other,
			z: self.z / other,
		}
	}
}

impl<T> Vec3<T>
where
	T: Mul<Output = T> + Add<Output = T>,
{
	pub fn dot(self, other: Self) -> T {
		self.x * other.x + self.y * other.y + self.z * other.z
	}
}

impl<T> Vec3<T>
where
	T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Copy,
{
	pub fn cross(self, other: Vec3<T>) -> Self {
		Self {
			x: self.y * other.z - self.z * other.y,
			y: self.z * other.x - self.x * other.z,
			z: self.x * other.y - self.y * other.x,
		}
	}
}

impl<T> From<[T; 3]> for Vec3<T>
where
	T: Float + SampleUniform + Copy,
{
	fn from(p: [T; 3]) -> Self {
		Self::new(p[0], p[1], p[2])
	}
}

pub type Vec3f = Vec3<f32>;
