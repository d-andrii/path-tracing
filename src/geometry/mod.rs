mod vector;
pub use vector::*;

mod polygon;
pub use polygon::*;

mod ray;
pub use ray::*;

mod object;
pub use object::*;

pub trait Intersect<T> {
	const EPSILON: f32 = 0.0000001;

	fn intersect(&self, other: &T) -> Option<Vec3f>;
}
