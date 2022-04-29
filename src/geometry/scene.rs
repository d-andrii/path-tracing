use std::cmp::Ordering;

use crate::image::Colour;

use super::{Intersect, Object, Polygon3, Ray, Vec3f};

pub struct Scene {
	objects: Vec<Box<dyn Object>>,
}

pub struct Hit<'a> {
	pub object: &'a Box<dyn Object>,
	pub polygon: &'a Polygon3,
	pub point: Vec3f,
}

impl Scene {
	pub fn new() -> Self {
		Self {
			objects: Vec::new(),
		}
	}

	pub fn add_object(&mut self, object: Box<dyn Object>) {
		self.objects.push(object);
	}

	pub fn hit(&self, ray: &Ray) -> Option<Hit> {
		let hit_objects = self
			.objects
			.iter()
			.filter(|object| object.bounding().intersect(ray).is_some());

		let hits = hit_objects.flat_map(|object| {
			object
				.faces()
				.iter()
				.filter_map(|polygon| polygon.intersect(ray).map(|point| (polygon, point)))
				.map(|(polygon, point)| Hit {
					object,
					polygon,
					point,
				})
		});

		hits.min_by(|x, y| {
			let d = (x.point - ray.origin).len() - (y.point - ray.origin).len();
			match (d < 0., d > 0.) {
				(false, true) => Ordering::Greater,
				(true, false) => Ordering::Less,
				_ => Ordering::Equal,
			}
		})
	}

	pub fn get_colour(&self, ray: &Ray, depth: usize) -> Colour {
		if depth == 0 {
			return Colour::from_rgb(0., 0., 0.);
		}

		if let Some(hit) = self.hit(ray) {
			if let Some(material) = hit.object.material() {
				let (scattered, colour) = material.get_scattered(ray, &hit);
				if let Some(scattered) = scattered {
					colour * self.get_colour(&scattered, depth - 1)
				} else {
					colour
				}
			} else {
				Colour::from_rgb(0., 0., 0.)
			}
		} else {
			let t = 0.5 * (ray.direction.y + 1.);
			Colour::from_rgb(1., 1., 1.) * (1. - t) + Colour::from_rgb(0.5, 0.7, 1.) * t
		}
	}
}
