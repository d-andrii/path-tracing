use rayon::prelude::*;

mod geometry;
mod image;
// mod progress;

use std::{f32::consts::PI, fs::File, io::Write};

use geometry::{Ray, SolidObject, Vec3f, WithOrigin, WithScale};
use image::{Colour, Image, ImageFormat};
use rand::Rng;

use crate::geometry::{Light, Scene};

fn main() {
	let samples_per_pixel = 50;
	let max_depth = 30;

	let width = 640;
	let height = 320;

	let aspect_ratio = (height as f32 - 1.) / (width as f32 - 1.);

	let model = "Avocado.glb".to_string();
	// let model = "Triangle.gltf".to_string();

	println!("Loading model");

	let mut model = SolidObject::from_gltf(model);
	model.scale(100.);
	model.move_to(Vec3f::new(0., 0., 15.));
	let mut ground = SolidObject::plane();
	ground.scale(10000.);
	ground.move_to(Vec3f::new(0., -4., 0.));
	let mut light = Light::plane();
	light.scale(8.);
	light.move_to(Vec3f::new(0., -3.5, 15.));
	let mut scene = Scene::new();
	scene.add_object(Box::new(model));
	scene.add_object(Box::new(ground));
	scene.add_object(Box::new(light));

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

	let pixels: Vec<_> = image
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

				let r = Ray::new(origin, p.unit());

				c = c + scene.get_colour(&r, max_depth);
			}

			let i = 1. / samples_per_pixel as f32;
			(x, y, (c * i).sqrt().clamp(0., 0.999))
		})
		.collect();

	println!("Setting image buffer");

	for (x, y, c) in pixels {
		image.set_pixel(x, y, c);
	}

	println!("Encoding image");
	let data = image::formats::Bmp::encode(&image).unwrap();

	let mut file = File::create("temp.bmp").unwrap();
	file.write_all(&data).unwrap();
}
