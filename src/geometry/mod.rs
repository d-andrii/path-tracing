mod vector;
pub use vector::*;

mod polygon;
pub use polygon::*;

mod ray;
pub use ray::*;

mod object;
pub use object::*;

mod scene;
pub use scene::*;

pub trait Intersect<T> {
	const EPSILON: f32 = 0.0001;

	fn intersect(&self, other: &T) -> Option<Vec3f>;
}
