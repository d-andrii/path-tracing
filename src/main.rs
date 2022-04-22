use rayon::prelude::*;

mod geometry;
mod image;
// mod progress;

use std::{cmp::Ordering, f32::consts::PI, fs::File, io::Write};

use geometry::{Intersect, Ray, SolidObject, Vec3f, WithOrigin, WithScale};
use image::{Colour, Image, ImageFormat};
use rand::Rng;

use crate::geometry::Scene;

fn main() {
	let samples_per_pixel = 10;

	let width = 640;
	let height = 320;

	let aspect_ratio = (height as f32 - 1.) / (width as f32 - 1.);

	let model = "Avocado.glb".to_string();
	// let model = "Triangle.gltf".to_string();

	println!("Loading model");

	let model = SolidObject::from_gltf(model)
		.scale(1000.)
		.move_to(Vec3f::new(0., 10., 200.));
	let ground = SolidObject::plane()
		.scale(10000.)
		.move_to(Vec3f::new(0., -40., 0.));
	let mut scene = Scene::new();
	scene.add_object(model);
	scene.add_object(ground);

	println!("Allocating image");

	let mut image = Image::new(width, height);

	let focus = Vec3f::new(0., 0., 20.);
	let origin = Vec3f::new(0., 0., 0.);
	let fov = PI / 2.;
	let d = 1.;

	let t = (focus - origin).unit();
	let v = Vec3f::new(1., 0., 0.).cross(t);
	let b = t.cross(v);

	let gx = d * (fov / 2.).tan();
	let gy = gx * aspect_ratio;

	let qx = b * ((2. * gx) / (width as f32 - 1.));
	let qy = v * ((2. * gy) / (height as f32 - 1.));

	let p0 = t * d - b * gx - v * gy;

	println!("Processing pixels");

	let pixesl: Vec<_> = image
		.coordinates()
		.par_bridge()
		.into_par_iter()
		.map(|(x, y)| {
			let mut rng = rand::thread_rng();
			let mut c = Colour::from_rgb(0., 0., 0.);

			for _ in 0..samples_per_pixel {
				let r1: f32 = rng.gen_range(0.0..1.0);
				let r2: f32 = rng.gen_range(0.0..1.0);
				let p = p0 + qx * (x as f32 + r1) + qy * (y as f32 + r2);

				let r = Ray {
					origin,
					direction: p.unit(),
				};

				// TODO: Make less confusing
				if let Some((_object, polygon, _point)) = scene
					.objects()
					.iter()
					.filter(|object| object.bounding.intersect(&r).is_some())
					.flat_map(|object| {
						object
							.faces
							.iter()
							.filter_map(|polygon| {
								polygon.intersect(&r).map(|point| (polygon, point))
							})
							.map(|(polygon, point)| (object.clone(), polygon, point))
					})
					.min_by(|(_, _, x), (_, _, y)| {
						let d = (*x - origin).len() - (*y - origin).len();
						match (d < 0., d > 0.) {
							(false, true) => Ordering::Greater,
							(true, false) => Ordering::Less,
							_ => Ordering::Equal,
						}
					}) {
					// c = o.colour.clone();
					c = c + Colour::from_rgb(
						polygon.normal.x + 1.,
						polygon.normal.y + 1.,
						polygon.normal.z + 1.,
					) * 0.5;
				} else {
					let t = 0.5 * (r.direction.y + 1.);
					c = c
						+ (Colour::from_rgb(1., 1., 1.) * (1. - t)
							+ Colour::from_rgb(0.5, 0.7, 1.) * t);
				}
			}

			let i = 1. / samples_per_pixel as f32;
			(x, y, (c * i).clamp(0., 0.999))
		})
		.collect();

	println!("Setting image buffer");

	for (x, y, c) in pixesl {
		image.set_pixel(x, y, c);
	}

	println!("Encoding image");
	let data = image::formats::Bmp::encode(&image).unwrap();

	let mut file = File::create("temp.bmp").unwrap();
	file.write_all(&data).unwrap();
}
