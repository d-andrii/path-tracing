use crate::{image::Colour, material::Material};

use super::{Intersect, Polygon3, Ray, Vec3f};

pub trait WithOrigin: Object {
	fn move_to(&mut self, origin: Vec3f) {
		let center = self.bounding().0 + (self.bounding().1 - self.bounding().0) / 2.;

		self.set_faces(
			self.faces()
				.iter()
				.map(|f| {
					Polygon3::new(
						f.a - center + origin,
						f.b - center + origin,
						f.c - center + origin,
					)
				})
				.collect(),
		);

		self.update_bounding_box();
	}
}

pub trait WithScale: Object {
	fn scale(&mut self, n: f32) {
		self.set_faces(self.faces().iter().map(|x| x.scale(n)).collect());

		self.set_bounding(BoundingBox(self.bounding().0 * n, self.bounding().1 * n));
	}

	fn scale_to(&mut self, x: f32, y: f32) {
		let size = self.bounding().1 - self.bounding().0;
		let min = f32::min(x / size.x, y / size.y);

		self.scale(min);
	}
}

pub trait Object: Sync + Send {
	fn faces(&self) -> &Vec<Polygon3>;
	fn bounding(&self) -> &BoundingBox;

	fn set_faces(&mut self, faces: Vec<Polygon3>);
	fn set_bounding(&mut self, bounding: BoundingBox);

	fn update_bounding_box(&mut self) {
		if self.faces().len() == 0 {
			self.set_bounding(BoundingBox(Vec3f::new(0., 0., 0.), Vec3f::new(0., 0., 0.)));
		} else {
			let mut b = BoundingBox(self.faces()[0].a, self.faces()[0].a);
			for f in self.faces() {
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
			self.set_bounding(b);
		}
	}

	fn material(&self) -> Option<&Material> {
		None
	}
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
	pub material: Material,
	pub faces: Vec<Polygon3>,
	pub bounding: BoundingBox,
}

impl WithOrigin for SolidObject {}
impl WithScale for SolidObject {}

impl Object for SolidObject {
	fn faces(&self) -> &Vec<Polygon3> {
		&self.faces
	}

	fn bounding(&self) -> &BoundingBox {
		&self.bounding
	}

	fn set_faces(&mut self, faces: Vec<Polygon3>) {
		self.faces = faces;
	}

	fn set_bounding(&mut self, bounding: BoundingBox) {
		self.bounding = bounding;
	}

	fn material(&self) -> Option<&Material> {
		Some(&self.material)
	}
}

impl SolidObject {
	pub fn from_gltf<S: AsRef<str>>(path: S) -> Self {
		let (gltf, buffers, _) = gltf::import(path.as_ref()).expect("Cannot open model");

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
			material: Material {
				albedo: Colour::from_rgb(0.8, 0.8, 0.8),
				specular: 0.,
				metalic: 0.,
				roughness: 0.,
				emission: 0.,
			},
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
			material: Material {
				albedo: Colour::from_rgb(0.6, 0.6, 0.6),
				specular: 0.,
				metalic: 0.,
				roughness: 0.,
				emission: 0.,
			},
		}
	}
}

#[derive(Debug, Clone)]
pub struct Light {
	pub faces: Vec<Polygon3>,
	pub bounding: BoundingBox,
	pub material: Material,
}

impl WithOrigin for Light {}
impl WithScale for Light {}

impl Object for Light {
	fn faces(&self) -> &Vec<Polygon3> {
		&self.faces
	}

	fn bounding(&self) -> &BoundingBox {
		&self.bounding
	}

	fn set_faces(&mut self, faces: Vec<Polygon3>) {
		self.faces = faces;
	}

	fn set_bounding(&mut self, bounding: BoundingBox) {
		self.bounding = bounding;
	}

	fn material(&self) -> Option<&Material> {
		Some(&self.material)
	}
}

impl Light {
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
			material: Material {
				albedo: Colour::from_rgb(1., 1., 1.),
				emission: 1.,
				metalic: 0.,
				roughness: 0.,
				specular: 0.,
			},
		}
	}
}
