use pixels::{Pixels, SurfaceTexture};
use rayon::prelude::*;
use winit::{
	dpi::LogicalSize,
	event::Event,
	event_loop::{ControlFlow, EventLoop},
	window::WindowBuilder,
};

mod geometry;
mod image;
mod material;
// mod progress;

use std::{
	f32::consts::PI,
	fs::File,
	io::Write,
	sync::{
		atomic::{AtomicBool, Ordering},
		Arc, RwLock,
	},
	thread,
};

use geometry::{Light, Ray, Scene, SolidObject, Vec3f, WithOrigin, WithScale};
use image::{Colour, Image, ImageFormat};
use rand::{prelude::SliceRandom, Rng};

const HEIGHT: u32 = 320;
const WIDTH: u32 = 640;
const SAMPLES_PER_PIXEL: i32 = 50;
const MAX_DEPTH: usize = 30;

const MODEL: &str = "Avocado.glb";

fn main() {
	let event_loop = EventLoop::new();
	let window = {
		let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
		WindowBuilder::new()
			.with_title("Hello Pixels")
			.with_inner_size(size)
			.with_min_inner_size(size)
			.build(&event_loop)
			.unwrap()
	};

	let mut pixels = {
		let window_size = window.inner_size();
		let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
		Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap()
	};

	let aspect_ratio = (HEIGHT as f32 - 1.) / (WIDTH as f32 - 1.);

	println!("Loading model");

	let mut model1 = SolidObject::from_gltf(MODEL);
	model1.scale(100.);
	model1.move_to(Vec3f::new(0., 0., 15.));
	let mut model2 = SolidObject::from_gltf(MODEL);
	model2.scale(100.);
	model2.move_to(Vec3f::new(-4.4, 0., 15.));
	model2.material.metalic = 1.;
	let mut ground = SolidObject::plane();
	ground.scale(10000.);
	ground.move_to(Vec3f::new(0., -4., 0.));
	let mut light = Light::plane();
	light.scale(8.);
	light.move_to(Vec3f::new(0., -3.5, 15.));
	let mut scene = Scene::new();
	scene.add_object(Box::new(model1));
	scene.add_object(Box::new(model2));
	scene.add_object(Box::new(ground));
	scene.add_object(Box::new(light));

	println!("Allocating image");

	let image = Arc::new(RwLock::new(Image::new(WIDTH, HEIGHT)));

	let focus = Vec3f::new(0., 0., 20.);
	let origin = Vec3f::new(0., 0., 0.);
	let fov = PI / 2.;
	let d = 1.;

	let t = (focus - origin).unit();
	let v = Vec3f::new(1., 0., 0.).cross(t);
	let b = t.cross(v);

	let gx = d * (fov / 2.).tan();
	let gy = gx * aspect_ratio;

	let qx = b * ((2. * gx) / (WIDTH as f32 - 1.));
	let qy = v * ((2. * gy) / (HEIGHT as f32 - 1.));

	let p0 = t * d - b * gx - v * gy;

	println!("Creating window");

	let running = Arc::new(AtomicBool::new(true));

	{
		let image = image.clone();
		let running = running.clone();
		thread::spawn(move || {
			println!("Processing pixels");

			let mut iter: Vec<(u32, u32)> = image.read().unwrap().coordinates().collect();
			iter.shuffle(&mut rand::thread_rng());

			iter.iter().par_bridge().into_par_iter().for_each(|(x, y)| {
				let mut rng = rand::thread_rng();
				let mut c = Colour::from_rgb(0., 0., 0.);

				for _ in 0..SAMPLES_PER_PIXEL {
					let r1: f32 = rng.gen_range(0.0..1.0);
					let r2: f32 = rng.gen_range(0.0..1.0);
					let p = p0 + qx * (*x as f32 + r1) + qy * (*y as f32 + r2);

					let r = Ray::new(origin, p.unit());

					c = c + scene.get_colour(&r, MAX_DEPTH) / SAMPLES_PER_PIXEL as f32;
				}

				image
					.write()
					.unwrap()
					.set_pixel(*x, *y, (c).sqrt().clamp(0., 0.999));

				window.request_redraw();
			});

			println!("Encoding image");
			let data = image::formats::Bmp::encode(&image.read().unwrap()).unwrap();

			let mut file = File::create("temp.bmp").unwrap();
			file.write_all(&data).unwrap();

			running.store(false, Ordering::Relaxed);
		});
	}

	event_loop.run(move |event, _, control_flow| {
		if let Event::RedrawRequested(_) = event {
			pixels
				.get_frame()
				.copy_from_slice(&image.read().unwrap().into_u8());
			if pixels.render().is_err() {
				*control_flow = ControlFlow::Exit;
				return;
			}

			if !running.load(Ordering::Relaxed) {
				*control_flow = ControlFlow::Exit;
				return;
			}
		}
	});
}
