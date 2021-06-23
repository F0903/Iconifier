use super::{png_meta::PngMetadata, Result};

pub enum ChunkType {
	Header,
	Palette,
	Data,
	DataEnd,
}

impl ChunkType {
	pub fn parse(name: &str) -> Result<ChunkType> {
		match name {
			"IHDR" => Ok(ChunkType::Header),
			"PLTE" => Ok(ChunkType::Palette),
			"IDAT" => Ok(ChunkType::Data),
			"IEND" => Ok(ChunkType::DataEnd),
			_ => Err("Unknown chunk type."),
		}
	}
}

pub enum ChunkData {
	Header(PngMetadata),
	Data(Vec<u8>),
}
