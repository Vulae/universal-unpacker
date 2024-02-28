
use std::{error::Error, io::Read};
use bitflags::bitflags;
use crate::util::read_ext::ReadExt;
use super::TextureError;



bitflags! {
    #[derive(Debug)]
    pub struct DataFormat: u32 {
        const MASK_IMAGE_FORMAT = (1 << 20) - 1;
        const PNG = 1 << 20;
        const WEBP = 1 << 21;
        const BIT_STREAM = 1 << 22;
        const HAS_MIPMAPS = 1 << 23;
        const DETECT_3D = 1 << 24;
        const DETECT_SRGB = 1 << 25;
        const DETECT_NORMAL = 1 << 26;
        const DETECT_ROUGHNESS = 1 << 27;
    }
}



pub struct V3Stream2d {
    width: u16,
    original_width: u16,
    height: u16,
    original_height: u16,
    flags: u32,
    data_format: DataFormat,
    num_mips: u32,
    mips: Vec<Vec<u8>>,
}

impl V3Stream2d {
    pub const IDENTIFIER: [u8; 4] = *b"GDST";

    pub fn load(data: &mut impl Read) -> Result<Self, Box<dyn Error>> {
        assert!(Self::IDENTIFIER.iter().eq(data.read_primitive::<[u8; 4]>()?.iter()), "Texture identifier does not match.");

        let width: u16 = data.read_primitive()?;
        let original_width: u16 = data.read_primitive()?;
        let height: u16 = data.read_primitive()?;
        let original_height: u16 = data.read_primitive()?;
        let flags: u32 = data.read_primitive()?;
        let data_format: DataFormat = DataFormat::from_bits_retain(data.read_primitive()?);

        let num_mips: u32;
        let mut mips: Vec<Vec<u8>> = Vec::new();

        if data_format.intersects(DataFormat::WEBP | DataFormat::PNG) {
            num_mips = data.read_primitive()?;

            for _ in 0..num_mips {
                if data_format.intersects(DataFormat::WEBP) {
                    let len: u32 = data.read_primitive()?;
                    assert!(b"WEBP".iter().eq(data.read_primitive::<[u8; 4]>()?.iter()), "Sub-texture identifier expected WEBP.");
                    let data = data.read_to_vec((len - 4) as usize)?;
                    mips.push(data);
                } else {
                    return Err(Box::new(TextureError::CannotRead));
                }
            }
        } else {
            return Err(Box::new(TextureError::CannotRead));
        }

        Ok(V3Stream2d {
            width,
            original_width,
            height,
            original_height,
            flags,
            data_format,
            num_mips,
            mips,
        })
    }

    pub fn to_image(&mut self) -> Result<(&str, Vec<u8>), Box<dyn Error>> {
        if self.data_format.intersects(DataFormat::PNG) {
            Ok(("png", self.mips[0].clone()))
        } else if self.data_format.intersects(DataFormat::WEBP) {
            Ok(("webp", self.mips[0].clone()))
        } else {
            Err(Box::new(TextureError::CannotConvert))
        }
    }
}


