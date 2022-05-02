use std::path::PathBuf;
use std::{error::Error, result::Result};

use ico_rs::png::write_ico;

fn main() -> Result<(), Box<dyn Error>> {
	let image = PathBuf::from(std::env::args().nth(1).ok_or("Image path was invalid.")?);
	let image_str = image.to_str().ok_or("Could not convert")?;
	write_ico(
		format!(
			"./{}",
			image
				.file_name()
				.ok_or("Could not get image file name.")?
				.to_str()
				.ok_or("Could not convert OsStr to str")?
		),
		image_str,
	)?;
	Ok(())
}
