mod ico_writer;
mod png_parser;

fn main() -> png_parser::Result<()> {
	let image = std::env::args().nth(1).ok_or("Image path was invalid.")?;
	let parser = png_parser::PngParser::new();
	let png = parser.parse_header(&image)?;
	ico_writer::write_ico("./", "icon", png, &image)?;
	Ok(())
}
