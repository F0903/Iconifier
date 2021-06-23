use super::{
	chunk::{ChunkData, ChunkType},
	png_meta::PngMetadata,
	Result,
};
use std::convert::TryInto;
use std::fs::read;

pub struct PngParser {}

impl PngParser {
	pub fn new() -> Self {
		PngParser {}
	}

	fn verify_signature(signature: [u8; 8]) -> bool {
		let hash = u64::from_be_bytes(signature);
		hash == 9894494448401390090
	}

	fn parse_header_chunk(header_data: &[u8]) -> Result<ChunkData> {
		let width = u32::from_be_bytes(
			header_data[0..4]
				.try_into()
				.map_err(|_| "Could not convert header data to array.")?,
		);
		let height = u32::from_be_bytes(
			header_data[4..8]
				.try_into()
				.map_err(|_| "Could not convert header data to array.")?,
		);
		let bit_depth = u8::from_be_bytes(
			header_data[8..9]
				.try_into()
				.map_err(|_| "Could not convert header data to array.")?,
		);
		let color_type = u8::from_be_bytes(
			header_data[9..10]
				.try_into()
				.map_err(|_| "Could not convert header data to array.")?,
		);
		let compression_method = u8::from_be_bytes(
			header_data[10..11]
				.try_into()
				.map_err(|_| "Could not convert header data to array.")?,
		);
		let filter_method = u8::from_be_bytes(
			header_data[11..12]
				.try_into()
				.map_err(|_| "Could not convert header data to array.")?,
		);
		let interlace_method = u8::from_be_bytes(
			header_data[12..13]
				.try_into()
				.map_err(|_| "Could not convert header data to array.")?,
		);
		Ok(ChunkData::Header(PngMetadata {
			width,
			height,
			bit_depth,
			color_type,
			compression_method,
			filter_method,
			interlace_method,
		}))
	}

	fn parse_data_chunk(chunk_data: &[u8]) -> Result<ChunkData> {
		let mut end_index = 0;
		chunk_data.iter().enumerate().for_each(|(i, x)| {
			if *x != b'I' {
				return;
			}
			if chunk_data[i + 1] != b'E' {
				return;
			}
			if chunk_data[i + 2] != b'N' {
				return;
			}
			if chunk_data[i + 3] != b'D' {
				return;
			}
			end_index = i + 3;
		});
		if end_index == 0 {
			return Err("Could not find end of data.");
		}
		Ok(ChunkData::Data(chunk_data[..end_index].to_vec()))
	}

	fn parse_chunk(chunk_data: &[u8], chunk_type: ChunkType) -> Result<ChunkData> {
		match chunk_type {
			ChunkType::Header => Self::parse_header_chunk(chunk_data),
			ChunkType::Data => Self::parse_data_chunk(chunk_data),
			_ => Err("Unknown chunk type was requested to parse."),
		}
	}

	pub fn parse_header(&self, file: impl AsRef<str>) -> Result<PngMetadata> {
		let data = read(file.as_ref())
			.map_err(|_| "Could not read image file. Have you entered the path correctly?")?;
		if !Self::verify_signature(data[..8].try_into().unwrap()) {
			return Err("Could not verify PNG signature.");
		}
		let data = &data[12..29]; // First 12 bytes are signature + padding.
		let chunk = unsafe { std::str::from_utf8_unchecked(&data[..4]) };
		println!("{}", chunk);
		if let Ok(ChunkType::Header) = ChunkType::parse(chunk) {
			if let Ok(ChunkData::Header(info)) = Self::parse_chunk(&data[4..], ChunkType::Header) {
				Ok(info)
			} else {
				Err("Could not parse header data.")
			}
		} else {
			Err("First chunk was not header. This is required.")
		}
	}

	// For decoding implementation
	/* pub fn parse(&self, file: impl AsRef<str>) -> Result<EncodedPng> {
		let file_data = read(file.as_ref())
			.map_err(|_| "Could not read image file. Have you entered the path correctly?")?;
		if !Self::verify_signature(file_data[..8].try_into().unwrap()) {
			return Err("Could not verify PNG signature.");
		}
		let mut png = EncodedPng::new();
		file_data.iter().skip(12).enumerate().for_each(|(i, _)| {
			let chunk = unsafe { std::str::from_utf8_unchecked(&file_data[i..i + 4]) };
			if let Ok(chunk_type) = ChunkType::parse(chunk) {
				match Self::parse_chunk(&file_data[i + 4..], chunk_type) {
					Ok(ChunkData::Header(info)) => png.info = info,
					Ok(ChunkData::Data(data)) => png.png_data = data,
					Err(_) => println!("Warning: failed to parse a chunk."),
				}
			}
		});
		Ok(png)
	} */
}
