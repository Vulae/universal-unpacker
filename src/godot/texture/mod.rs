
use std::{error::Error, fmt, io::{Read, Seek}};
use self::v4compressed2d::V4Compressed2d;

mod v4compressed2d;



#[derive(Debug, Clone)]
pub enum TextureError {
    UnknownFormat,
    CannotRead,
    CannotConvert,
}

impl fmt::Display for TextureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownFormat => write!(f, "Texture format not recognized."),
            Self::CannotRead => write!(f, "Cannot read texture."),
            Self::CannotConvert => write!(f, "Cannot convert texture to image."),
        }
    }
}

impl Error for TextureError { }



pub enum Texture {
    V2Texture,
    V2ImageTexture,
    V2AtlasTexture,
    V2LargeTexture,
    V2Cubemap,
    V3Stream2d,
    V3Stream3d,
    V3StreamArray,
    V4Compressed2d(V4Compressed2d),
    V4Compressed3d,
    V4CompressedLayered,
}

impl Texture {
    pub fn load(mut data: impl Read + Seek) -> Result<Self, Box<dyn Error>> {
        let mut identifier = [0u8; 4];
        data.read(&mut identifier)?;
        data.seek(std::io::SeekFrom::Start(0))?;

        match &identifier {
            &V4Compressed2d::IDENTIFIER => Ok(Texture::V4Compressed2d(V4Compressed2d::load(data)?)),
            _ => Err(Box::new(TextureError::UnknownFormat)),
        }
    }

    pub fn to_image(&mut self) -> Result<(&str, Vec<u8>), Box<dyn Error>> {
        match self {
            Texture::V4Compressed2d(texture) => texture.to_image(),
            _ => Err(Box::new(TextureError::UnknownFormat)),
        }
    }
}


