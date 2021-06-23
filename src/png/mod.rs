mod chunk;
mod encoded_png;
pub(crate) mod png_meta;
mod png_parser;

pub use png_parser::PngParser;

pub type Result<T> = std::result::Result<T, &'static str>;
