mod ico_writer;
mod png;

use std::path::PathBuf;

fn main() -> png::Result<()> {
	let image = PathBuf::from(std::env::args().nth(1).ok_or("Image path was invalid.")?);
	let image_str = image.to_str().unwrap();
	let parser = png::PngParser::new();
	let png = parser.parse_header(image_str)?;
	ico_writer::write_ico(
		format!("./{}", image.file_name().unwrap().to_str().unwrap()),
		png,
		image_str,
	)?;
	Ok(())
}
