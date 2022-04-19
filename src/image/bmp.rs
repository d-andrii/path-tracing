use std::io::{self, Write};

pub struct Bmp {}

impl super::ImageFormat for Bmp {
	fn encode(image: &super::Image) -> io::Result<Vec<u8>> {
		let extra_bytes = image.width % 4;
		let pixel_array_size = image.height * (image.width * 3 + extra_bytes);
		let offset = 54;
		let mut data: Vec<u8> = Vec::with_capacity((pixel_array_size + offset) as usize);

		data.write(&[b'B', b'M'])?;
		data.write(&(pixel_array_size + offset).to_le_bytes())?;
		data.write(&(0 as u32).to_le_bytes())?;
		data.write(&(offset as u32).to_le_bytes())?;
		data.write(&(40 as u32).to_le_bytes())?;
		data.write(&(image.width as i32).to_le_bytes())?;
		data.write(&(-(image.height as i32)).to_le_bytes())?;
		data.write(&(1 as u16).to_le_bytes())?;
		data.write(&(24 as u16).to_le_bytes())?;
		data.write(&(0 as u32).to_le_bytes())?;
		data.write(&(pixel_array_size).to_le_bytes())?;
		data.write(&(0 as u128).to_le_bytes())?;

		data.resize((pixel_array_size + offset) as usize, 0);

		let row_bytes = image.width * 3 + extra_bytes;

		for (x, y) in image.coordinates() {
			let colour = image.get_pixel(x, y);
			let p = (offset + y * row_bytes + x * 3) as usize;
			data[p] = (255. * colour.b) as u8;
			data[p + 1] = (255. * colour.g) as u8;
			data[p + 2] = (255. * colour.r) as u8;

			if x == 0 && y != 0 && extra_bytes > 0 {
				let fill_offset = (offset + y * row_bytes + image.width * 3) as usize;
				for i in fill_offset..(fill_offset + extra_bytes as usize) {
					data[i] = 0;
				}
			}
		}

		Ok(data)
	}
}
