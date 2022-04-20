use crate::image::Colour;

use super::{Intersect, Polygon3, Ray, Vec3f};

pub trait WithOrigin {
	fn move_to(self, origin: Vec3f) -> Self;
}

pub trait WithScale {
	fn scale(self, n: f32) -> Self;
	fn scale_to(self, x: f32, y: f32) -> Self;
}

#[derive(Debug, Clone)]
pub struct BoundingBox(Vec3f, Vec3f);

impl Intersect<Ray> for BoundingBox {
	fn intersect(&self, r: &Ray) -> Option<Vec3f> {
		let (mut tmin, mut tmax, tymin, tymax, tzmin, tzmax): (f32, f32, f32, f32, f32, f32);

		let divx = 1. / r.direction.x;

		if divx >= 0. {
			tmin = (self.0.x - r.origin.x) * divx;
			tmax = (self.1.x - r.origin.x) * divx;
		} else {
			tmin = (self.1.x - r.origin.x) * divx;
			tmax = (self.0.x - r.origin.x) * divx;
		}

		if r.direction.y >= 0. {
			tymin = (self.0.y - r.origin.y) / r.direction.y;
			tymax = (self.1.y - r.origin.y) / r.direction.y;
		} else {
			tymin = (self.1.y - r.origin.y) / r.direction.y;
			tymax = (self.0.y - r.origin.y) / r.direction.y;
		}

		if tmin > tymax || tymin > tmax {
			return None;
		}

		if tymin > tmin {
			tmin = tymin
		}

		if tymax < tmax {
			tmax = tymax
		}

		if r.direction.z >= 0. {
			tzmin = (self.0.z - r.origin.z) / r.direction.z;
			tzmax = (self.1.z - r.origin.z) / r.direction.z;
		} else {
			tzmin = (self.1.z - r.origin.z) / r.direction.z;
			tzmax = (self.0.z - r.origin.z) / r.direction.z;
		}

		if tmin > tzmax || tzmin > tmax {
			return None;
		}

		Some(self.0)
	}
}

#[derive(Debug, Clone)]
pub struct SolidObject {
	pub faces: Vec<Polygon3>,
	pub bounding: BoundingBox,
	pub colour: Colour,
}

impl WithOrigin for SolidObject {
	fn move_to(mut self, origin: Vec3f) -> Self {
		let center = self.bounding.0 + (self.bounding.1 - self.bounding.0) / 2.;

		for mut f in &mut self.faces {
			f.a = f.a - center + origin;
			f.b = f.b - center + origin;
			f.c = f.c - center + origin;
		}

		self.update_bounding_box();

		self
	}
}

impl WithScale for SolidObject {
	fn scale(mut self, n: f32) -> Self {
		self.faces = self.faces.into_iter().map(|x| x.scale(n)).collect();

		self.bounding.0 = self.bounding.0 * n;
		self.bounding.1 = self.bounding.1 * n;

		// TODO: Check if multiplication is good enough

		// self.update_bounding_box();

		self
	}

	fn scale_to(self, x: f32, y: f32) -> Self {
		let size = self.bounding.1 - self.bounding.0;
		let min = f32::min(x / size.x, y / size.y);

		self.scale(min)
	}
}

impl SolidObject {
	fn update_bounding_box(&mut self) {
		if self.faces.len() == 0 {
			self.bounding = BoundingBox(Vec3f::new(0., 0., 0.), Vec3f::new(0., 0., 0.));
		} else {
			let mut b = BoundingBox(self.faces[0].a, self.faces[0].a);
			for f in &self.faces {
				for p in [f.a, f.b, f.c] {
					if p.x < b.0.x {
						b.0.x = p.x
					}
					if p.y < b.0.y {
						b.0.y = p.y
					}
					if p.z < b.0.z {
						b.0.z = p.z
					}
					if p.x > b.1.x {
						b.1.x = p.x
					}
					if p.y > b.1.y {
						b.1.y = p.y
					}
					if p.z > b.1.z {
						b.1.z = p.z
					}
				}
			}
			self.bounding = b;
		}
	}

	pub fn from_gltf(path: String) -> Self {
		let (gltf, buffers, _) = gltf::import(path).expect("Cannot open model");

		let mesh = gltf.meshes().next().expect("No mesh in model");
		let primitive = mesh.primitives().next().expect("No primitive in model");
		let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

		let mut faces: Vec<Polygon3> = Vec::new();
		let positions: Vec<[f32; 3]> = reader
			.read_positions()
			.map(|iter| iter.collect())
			.expect("No positions in model");
		let indices: Vec<u32> = reader
			.read_indices()
			.map(|iter| iter.into_u32().collect())
			.expect("No indices in model");

		for i in indices.chunks_exact(3) {
			faces.push(Polygon3::new(
				positions[i[0] as usize].into(),
				positions[i[1] as usize].into(),
				positions[i[2] as usize].into(),
			));
		}

		let bound = primitive.bounding_box();

		Self {
			faces,
			bounding: BoundingBox(bound.min.into(), bound.max.into()),
			colour: Colour::from_rgb(0.95, 0.95, 0.95),
		}
	}

	pub fn plane() -> Self {
		Self {
			faces: vec![
				Polygon3::new(
					Vec3f::new(0., 0., 0.),
					Vec3f::new(1., 0., 0.),
					Vec3f::new(0., 0., 1.),
				),
				Polygon3::new(
					Vec3f::new(1., 0., 1.),
					Vec3f::new(1., 0., 0.),
					Vec3f::new(0., 0., 1.),
				),
			],
			bounding: BoundingBox(Vec3f::new(0., 0., 0.), Vec3f::new(1., 0., 1.)),
			colour: Colour::from_rgb(0.2, 0.2, 0.2),
		}
	}
}

pub struct Scene {
	objects: Vec<SolidObject>,
}

impl Scene {
	pub fn new() -> Self {
		Self {
			objects: Vec::new(),
		}
	}

	pub fn add_object(&mut self, object: SolidObject) {
		println!("{:#?}", object);
		self.objects.push(object);
	}

	pub fn objects(&self) -> Vec<SolidObject> {
		self.objects.clone()
	}
}
