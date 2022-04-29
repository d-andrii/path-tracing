use crate::{
	geometry::{Hit, Ray, Vec3f},
	image::Colour,
};

pub type ReflectedRay = (Option<Ray>, Colour);

#[derive(Debug, Clone)]
pub struct Material {
	pub albedo: Colour,
	pub specular: f32,
	pub metalic: f32,
	pub roughness: f32,
	pub emission: f32,
}

impl Material {
	pub fn get_scattered(&self, ray: &Ray, hit: &Hit) -> ReflectedRay {
		// TODO: Mix different types of materials

		if self.emission == 1. {
			return (None, self.albedo.clone());
		}

		if self.metalic != 0. {
			let reflected = ray.direction.unit().reflect(hit.polygon.normal);
			let ray_out = Ray::new(hit.point, reflected);
			return (
				if ray_out.direction.dot(hit.polygon.normal) > 0. {
					Some(ray_out)
				} else {
					None
				},
				self.albedo.clone(),
			);
		}

		#[cfg(feature = "hemi_shading")]
		let t = {
			let r = Vec3f::rand_in_unit();
			hit.point
				+ if r.dot(hit.polygon.normal) > 0. {
					r
				} else {
					r * -1.
				}
		};
		#[cfg(not(feature = "hemi_shading"))]
		let t = hit.point + hit.polygon.normal + Vec3f::rand_in_unit();

		(
			Some(Ray::new(hit.point, t - hit.point)),
			self.albedo.clone(),
		)
	}
}
