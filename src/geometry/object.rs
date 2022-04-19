use crate::image::Colour;

use super::{Polygon3, Vec3f};

pub trait WithOrigin {
	fn move_to(self, origin: Vec3f) -> Self;
}

pub trait WithScale {
	fn scale(self, n: f32) -> Self;
	fn scale_to(self, x: f32, y: f32) -> Self;
}

#[derive(Debug, Clone)]
pub struct SolidObject {
	pub faces: Vec<Polygon3>,
	pub bounding: (Vec3f, Vec3f),
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

		self.bounding.0 = self.bounding.0 + origin;
		self.bounding.1 = self.bounding.1 + origin;

		self
	}
}

impl WithScale for SolidObject {
	fn scale(mut self, n: f32) -> Self {
		self.faces = self.faces.into_iter().map(|x| x.scale(n)).collect();

		self.bounding.0 = self.bounding.0 * n;
		self.bounding.1 = self.bounding.1 * n;

		self
	}

	fn scale_to(self, x: f32, y: f32) -> Self {
		let size = self.bounding.1 - self.bounding.0;
		let min = f32::min(x / size.x, y / size.y);

		println!("{:?} {}", self.bounding, min);

		self.scale(min)
	}
}

impl SolidObject {
	pub fn from_gltf(path: String) -> Self {
		let (gltf, buffers, _) = gltf::import(path).expect("Cannot open model");

		
		let mesh = gltf.meshes().next().expect("No mesh in model");
		let primitive = mesh.primitives().next().expect("No primitive in model");
		let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

		let mut faces: Vec<Polygon3> = Vec::new();
		if let Some(iter) = reader.read_positions() {
			let p: Vec<[f32; 3]> = iter.collect();
			for f in p.chunks_exact(3) {
				faces.push(Polygon3::new(f[0].into(), f[1].into(), f[2].into()));
			}
		}

		let bound = primitive.bounding_box();

		Self {
			faces,
			bounding: (bound.min.into(), bound.max.into()),
			colour: Colour::from_rgb(0.9, 0.9, 0.9),
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
			bounding: (Vec3f::new(0., 0., 0.), Vec3f::new(1., 0., 1.)),
			colour: Colour::from_rgb(0.6, 0.6, 0.6),
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
		self.objects.push(object);
	}

	pub fn objects(&self) -> Vec<SolidObject> {
		self.objects.clone()
	}
}
