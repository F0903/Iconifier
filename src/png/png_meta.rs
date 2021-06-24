#[derive(Debug)]
pub struct PngMetadata {
	pub x: u32,
	pub y: u32,
	pub bit_depth: u8,
	pub color_type: u8,
	pub compression_method: u8,
	pub filter_method: u8,
	pub interlace_method: u8,
}

// For decoding implementation
/* impl PngMetadata {
	pub fn zero() -> Self {
		PngMetadata {
			width: 0,
			height: 0,
			bit_depth: 0,
			color_type: 0,
			compression_method: 0,
			filter_method: 0,
			interlace_method: 0,
		}
	}
} */
