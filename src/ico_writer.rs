use crate::png::{png_meta::PngMetadata, Result};
use std::path::PathBuf;

pub fn write_ico(
	path: impl AsRef<str>,
	filename: impl AsRef<str>,
	png: PngMetadata,
	png_path: impl AsRef<str>,
) -> Result<()> {
	if png.width > 256 {
		return Err("Image width cannot be more than 256px.");
	}
	if png.height > 256 {
		return Err("Image height cannot be more than 256px.");
	}

	let path = path.as_ref();
	let filename = filename.as_ref();
	let mut path = PathBuf::from(path);
	path.push(filename);
	path.set_extension("ico");
	let mut buf = Vec::<u8>::new();
	let mut png_file = std::fs::read(png_path.as_ref()).map_err(|_| "Could not open png file.")?;

	//ICONDIR
	buf.extend(0u16.to_le_bytes());
	buf.extend(1u16.to_le_bytes());
	buf.extend(1u16.to_le_bytes());
	//ICONDIRENTRY
	buf.push(if png.width == 256 { 0 } else { png.width as u8 });
	buf.push(if png.height == 256 {
		0
	} else {
		png.height as u8
	});
	buf.push(0u8);
	buf.push(0u8);
	buf.extend(1u16.to_le_bytes());
	buf.extend((png.bit_depth as u16).to_le_bytes());
	buf.extend((png_file.len() as u32).to_le_bytes());
	buf.extend((buf.len() as u32 + 4).to_le_bytes());
	buf.append(&mut png_file);
	std::fs::write(path, buf).map_err(|_| "Could not write icon to disk. Is the path valid?")?;
	Ok(())
}
