
use std::{error::Error, io::{Read, Seek}};
use bitflags::bitflags;
use crate::util::read_ext::ReadExt;

use super::TextureError;



bitflags! {
    pub struct DataFlags: u32 {
        const BIT_STREAM = 1 << 22;
        const HAS_MIPMAPS = 1 << 23;
        const DETECT_3D = 1 << 24;
        const DETECT_SRGB = 1 << 25;
        const DETECT_NORMAL = 1 << 26;
        const DETECT_ROUGHNESS = 1 << 27;
    }
}



#[derive(PartialEq)]
enum DataFormat {
    Image,
    Png,
    Webp,
    BasisUniversal
}

impl DataFormat {
    pub fn from(v: u32) -> Self {
        match v {
            0 => DataFormat::Image,
            1 => DataFormat::Png,
            2 => DataFormat::Webp,
            3 => DataFormat::BasisUniversal,
            _ => panic!("Failed to convert data format."),
        }
    }
}



pub struct V4Compressed2d {
    original_width: u32,
    original_height: u32,
    flags: DataFlags,
    original_num_mips: i32,
    data_format: DataFormat,
    width: u16,
    height: u16,
    num_mips: u32,
    format: u32,
    mips: Vec<Vec<u8>>,
}

impl V4Compressed2d {
    pub const IDENTIFIER: [u8; 4] = *b"GST2";

    pub fn load(data: &mut (impl Read + Seek)) -> Result<Self, Box<dyn Error>> {
        assert!(Self::IDENTIFIER.iter().eq(data.read_primitive::<[u8; 4]>()?.iter()), "Texture identifier does not match.");
        assert!(data.read_primitive::<u32>()? == 1, "Texture version must be 1.");
        let original_width: u32 = data.read_primitive()?;
        let original_height: u32 = data.read_primitive()?;
        let flags = DataFlags::from_bits_retain(data.read_primitive()?);
        let original_num_mips: i32 = data.read_primitive()?;
        data.seek(std::io::SeekFrom::Current(3 * 4))?;

        let data_format = DataFormat::from(data.read_primitive()?);
        let width: u16 = data.read_primitive()?;
        let height: u16 = data.read_primitive()?;
        let num_mips: u32 = data.read_primitive()?;
        let format: u32 = data.read_primitive()?;

        let mut mips: Vec<Vec<u8>> = Vec::new();

        match data_format {
            DataFormat::Png | DataFormat::Webp => {
                for _ in 0..=num_mips {
                    let len: u32 = data.read_primitive()?;
                    let data = data.read_to_vec(len as usize)?;
                    mips.push(data);
                }
            },
            _ => return Err(Box::new(TextureError::CannotRead)),
        }

        Ok(V4Compressed2d {
            original_width,
            original_height,
            flags,
            original_num_mips,
            data_format,
            width,
            height,
            num_mips,
            format,
            mips,
        })
    }

    pub fn to_image(&mut self) -> Result<(&str, Vec<u8>), Box<dyn Error>> {
        match self.data_format {
            DataFormat::Image => Err(Box::new(TextureError::CannotConvert)),
            DataFormat::Png => Ok(("png", self.mips[0].clone())),
            DataFormat::Webp => Ok(("webp", self.mips[0].clone())),
            DataFormat::BasisUniversal => Err(Box::new(TextureError::CannotConvert)),
        }
    }
}


