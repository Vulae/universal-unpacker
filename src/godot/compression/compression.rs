
use core::fmt;
use std::{error::Error, io::{Cursor, Read}};



#[derive(Debug, Clone)]
enum CompressionError {
    UnsupportedCompressionMethod(Compression),
    InvalidCompressionMethod(u32),
    ZSTDDecompressionFailed(usize),
}

impl fmt::Display for CompressionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedCompressionMethod(method) => write!(f, "Unsupported compression method {:#?}", method),
            Self::InvalidCompressionMethod(method) => write!(f, "Invalid compression method {}", method),
            Self::ZSTDDecompressionFailed(code) => write!(f, "ZSTD decompression failed. {}", code),
        }
    }
}

impl Error for CompressionError { }





#[derive(Debug, Clone)]
pub struct CompressionZSTD {

}

impl CompressionZSTD {
    pub fn decompress(&mut self, data: Vec<u8>) -> Result<Vec<u8>, Box<dyn Error>> {
        // let mut ctx = zstd::zstd_safe::DCtx::create();
        // let mut decompressed: Vec<u8> = Vec::new();
        // decompressed.resize(4096, 0);
        // println!("Data Len: {}", data.len());
        // match ctx.decompress(&mut decompressed, &data) {
        //     Ok(_) => Ok(decompressed),
        //     Err(code) => Err(Box::new(CompressionError::ZSTDDecompressionFailed(code))),
        // }

        println!("Data Len: {}", data.len());
        println!("{:#?}", &data[0..16]);
        let mut decoder = ruzstd::StreamingDecoder::new(Cursor::new(data))?;
        let mut result: Vec<u8> = Vec::new();
        decoder.read_to_end(&mut result)?;

        println!("{}", String::from_utf8_lossy(&result));

        Ok(result)
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


