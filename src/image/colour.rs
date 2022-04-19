use std::ops::{Add, Div, Mul, Sub};

#[derive(Default, Clone, Debug)]
pub struct Colour {
	pub r: f32,
	pub g: f32,
	pub b: f32,
}

impl Colour {
	pub fn from_rgb(r: f32, g: f32, b: f32) -> Self {
		Self { r, g, b }
	}

	pub fn new() -> Self {
		Self::default()
	}
}

fn zip_with<F>(f: F, a: Colour, b: Colour) -> Colour
where
	F: Fn(f32, f32) -> f32,
{
	Colour {
		r: f(a.r, b.r),
		g: f(a.g, b.g),
		b: f(a.b, b.b),
	}
}

impl Add for Colour {
	type Output = Self;

	fn add(self, other: Self) -> Self {
		zip_with(f32::add, self, other)
	}
}

impl Sub for Colour {
	type Output = Self;

	fn sub(self, other: Self) -> Self {
		zip_with(f32::sub, self, other)
	}
}

impl Mul for Colour {
	type Output = Self;

	fn mul(self, other: Self) -> Self {
		zip_with(f32::mul, self, other)
	}
}

impl Div for Colour {
	type Output = Self;

	fn div(self, other: Self) -> Self {
		zip_with(f32::div, self, other)
	}
}
