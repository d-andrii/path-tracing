mod geometry;
mod image;
mod progress;

use std::{cmp::Ordering, f32::consts::PI, fs::File, io::Write};

use geometry::{Intersect, Ray, SolidObject, Vec3f, WithOrigin, WithScale};
use image::{Colour, Image, ImageFormat};
use progress::Progress;

use crate::geometry::Scene;

fn main() {
	let width = 200;
	let height = 100;

	let aspect_ratio = (height as f32 - 1.) / (width as f32 - 1.);

	let model = "Avocado.glb".to_string();
	// let model = "Triangle.gltf".to_string();

	println!("Loading model");

	let model = SolidObject::from_gltf(model)
		.scale_to(width as f32, height as f32)
		.move_to(Vec3f::new(0., 10., 200.));
	let ground = SolidObject::plane()
		.scale(10000.)
		.move_to(Vec3f::new(0., -10., 0.));
	let mut scene = Scene::new();
	scene.add_object(model);
	scene.add_object(ground);

	println!("Allocating image");

	let mut image = Image::new(width, height);
	let mut progress = Progress::new(width * height);

	let focus = Vec3f::new(0., 0., 20.);
	let origin = Vec3f::new(0., 0., 0.);
	let fov = PI / 2.;
	let d = 1.;

	let t = (focus - origin).unit();
	let v = Vec3f::new(1., 0., 0.).cross(t);
	let b = t.cross(v);

	let gx = d * (fov / 2.).tan();
	let gy = gx * aspect_ratio;

	for (x, y) in image.coordinates() {
		let qx = b * ((2. * gx) / (width as f32 - 1.));
		let qy = v * ((2. * gy) / (height as f32 - 1.));

		let p0 = t * d - b * gx - v * gy;
		let p = p0 + qx * x as f32 + qy * y as f32;

		let r = Ray {
			origin,
			direction: p.unit(),
		};
		let t = 0.5 * (r.direction.y + 1.);
		let m = 1. - t;
		let mut c = Colour::from_rgb(m, m, m) * Colour::from_rgb(1., 1., 1.)
			+ Colour::from_rgb(t, t, t) * Colour::from_rgb(0.5, 0.7, 1.);

		// TODO: Use bounding box for speed-up
		if let Some((o, _)) = scene
			.objects()
			.iter()
			.flat_map(|object| {
				object
					.faces
					.iter()
					.filter_map(|z| z.intersect(&r))
					.map(|z| (object.clone(), z))
			})
			.min_by(|(_, x), (_, y)| {
				let d = (*x - origin).len() - (*y - origin).len();
				match (d < 0., d > 0.) {
					(false, true) => Ordering::Greater,
					(true, false) => Ordering::Less,
					_ => Ordering::Equal,
				}
			}) {
			c = o.colour.clone();
		}

		image.set_pixel(x, y, c);

		progress.next();
		print!("\rProcessing pixels: {}", progress);
	}
	println!();

	println!("Encoding image");
	let data = image::formats::Bmp::encode(&image).unwrap();

	let mut file = File::create("temp.bmp").unwrap();
	file.write_all(&data).unwrap();
}
