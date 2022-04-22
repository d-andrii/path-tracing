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

	pub fn clamp(self, min: f32, max: f32) -> Self {
		Self {
			r: self.r.clamp(min, max),
			g: self.g.clamp(min, max),
			b: self.b.clamp(min, max),
		}
	}
}

macro_rules! impl_f32_math {
	($f:tt, $fn:tt) => {
		impl $f<f32> for Colour {
			type Output = Self;

			fn $fn(self, other: f32) -> Self {
				Self {
					r: f32::$fn(self.r, other),
					g: f32::$fn(self.g, other),
					b: f32::$fn(self.b, other),
				}
			}
		}
	};
}

macro_rules! impl_self_math {
	($f:tt, $fn:tt) => {
		impl $f<Colour> for Colour {
			type Output = Self;

			fn $fn(self, other: Colour) -> Self {
				Self {
					r: f32::$fn(self.r, other.r),
					g: f32::$fn(self.g, other.g),
					b: f32::$fn(self.b, other.b),
				}
			}
		}
	};
}

impl_f32_math!(Add, add);
impl_f32_math!(Sub, sub);
impl_f32_math!(Mul, mul);
impl_f32_math!(Div, div);

impl_self_math!(Add, add);
impl_self_math!(Sub, sub);
impl_self_math!(Mul, mul);
impl_self_math!(Div, div);
