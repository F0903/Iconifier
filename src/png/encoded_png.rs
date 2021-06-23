use super::png_meta::PngMetadata;

#[derive(Debug)]
pub struct EncodedPng {
	pub info: PngMetadata,
	pub png_data: Vec<u8>,
}

// For decoding implementation
/* impl EncodedPng {
	pub fn new() -> Self {
		EncodedPng {
			info: PngMetadata::zero(),
			png_data: Vec::new(),
		}
	}
} */
