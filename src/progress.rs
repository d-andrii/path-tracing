use std::fmt::Display;

use num::Integer;

pub struct Progress<T: Integer> {
	i: T,
	count: T,
}

impl<T> Progress<T>
where
	T: Integer + Copy,
{
	pub fn new(count: T) -> Self {
		Self {
			i: T::zero(),
			count,
		}
	}

	pub fn next(&mut self) {
		self.i = self.i + T::one();
	}

	pub fn get(&self) -> (T, T) {
		(self.i, self.count)
	}
}

impl<T> Display for Progress<T>
where
	T: Integer + Copy + Display,
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let (i, j) = self.get();
		write!(f, "{} of {}", i, j)
	}
}
