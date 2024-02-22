
use core::fmt;
use std::{error::Error, io::Cursor};



#[derive(Debug, Clone)]
enum CompressionError {
    UnsupportedCompressionMethod(Compression),
    InvalidCompressionMethod(u32),
}

impl fmt::Display for CompressionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedCompressionMethod(method) => write!(f, "Unsupported compression method {:#?}", method),
            Self::InvalidCompressionMethod(method) => write!(f, "Invalid compression method {}", method),
        }
    }
}

impl Error for CompressionError { }





#[derive(Debug, Clone)]
pub struct CompressionZSTD {

}

impl CompressionZSTD {
    pub fn decompress(&mut self, data: Vec<u8>) -> Result<Vec<u8>, Box<dyn Error>> {
        let decompressed = zstd::stream::decode_all(Cursor::new(data))?;
        Ok(decompressed)
    }
}



#[derive(Debug, Clone)]
pub enum Compression {
    FastLZ,
    Deflate,
    ZSTD(CompressionZSTD),
    GZIP,
    Brotli,
}

impl Compression {

    pub fn from(v: u32) -> Result<Compression, Box<dyn Error>> {
        Ok(match v {
            0 => return Err(Box::new(CompressionError::UnsupportedCompressionMethod(Compression::FastLZ))),
            1 => return Err(Box::new(CompressionError::UnsupportedCompressionMethod(Compression::Deflate))),
            2 => Compression::ZSTD(CompressionZSTD { }),
            3 => return Err(Box::new(CompressionError::UnsupportedCompressionMethod(Compression::GZIP))),
            4 => return Err(Box::new(CompressionError::UnsupportedCompressionMethod(Compression::Brotli))),
            v => return Err(Box::new(CompressionError::InvalidCompressionMethod(v))),
        })
    }

    pub fn decompress(&mut self, data: Vec<u8>) -> Result<Vec<u8>, Box<dyn Error>> {
        match self {
            Compression::ZSTD(method) => method.decompress(data),
            _ => panic!(),
        }
    }

}


