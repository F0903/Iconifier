use crate::png::{png_meta::PngMetadata, Result};
use std::path::{Path, PathBuf};

fn write_icon_dir(buf: &mut Vec<u8>) {
	buf.extend(0u16.to_le_bytes()); // Reserved
	buf.extend(1u16.to_le_bytes()); // Image type
	buf.extend(1u16.to_le_bytes()); // Image count
}

fn write_icon_dir_entry(buf: &mut Vec<u8>, png: PngMetadata, png_path: &Path) -> Result<()> {
	buf.push(if png.x == 256 { 0 } else { png.x as u8 }); // Image width
	buf.push(if png.y == 256 { 0 } else { png.y as u8 }); // Image height
	buf.push(0u8); // Color count
	buf.push(0u8); // Reserved
	buf.extend(1u16.to_le_bytes()); // Color planes
	buf.extend((png.bit_depth as u16).to_le_bytes()); // Bits per pixel
	let mut png_file = std::fs::read(png_path).map_err(|_| "Could not open png file.")?;
	buf.extend((png_file.len() as u32).to_le_bytes()); // Image data size
	buf.extend((buf.len() as u32 + 4).to_le_bytes()); // Image offset from file start
	buf.append(&mut png_file); // Image data
	Ok(())
}

pub fn write_ico(
	out_path: impl AsRef<Path>,
	png: PngMetadata,
	png_path: impl AsRef<Path>,
) -> Result<()> {
	if png.x > 256 {
		return Err("Image width cannot be more than 256px.");
	}
	if png.y > 256 {
		return Err("Image height cannot be more than 256px.");
	}

	let mut buf = Vec::<u8>::new();
	write_icon_dir(&mut buf);
	write_icon_dir_entry(&mut buf, png, png_path.as_ref())?;

	let mut path = PathBuf::from(out_path.as_ref());
	path.set_extension("ico");
	std::fs::write(path, buf).map_err(|_| "Could not write icon to disk. Is the path valid?")?;
	Ok(())
}
