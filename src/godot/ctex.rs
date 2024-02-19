
use std::{error::Error, io::Read};
use bitstream_io::{ByteRead, ByteReader, LittleEndian};
use bitflags::bitflags;



bitflags! {
    pub struct GodotCtexFlags: u32 {
        const BIT_STREAM = 1 << 22;
        const HAS_MIPMAPS = 1 << 23;
        const DETECT_3D = 1 << 24;
        const DETECT_NORMAL = 1 << 26;
        const DETECT_ROUGHNESS = 1 << 27;
    }
}



pub struct GodotCtex {
    pub original_width: u32,
    pub original_height: u32,
    pub flags: GodotCtexFlags,
    pub original_num_mips: i32,
    pub data_format: GodotCtexDataFormats,
    pub width: u16,
    pub height: u16,
    pub num_mips: u32,
    pub format: u32,
    pub mips: Vec<Vec<u8>>,
}



#[derive(PartialEq)]
pub enum GodotCtexDataFormats {
    Image,
    Png,
    Webp,
    BasisUniversal
}

impl GodotCtexDataFormats {
    pub fn from(v: u32) -> Self {
        match v {
            0 => GodotCtexDataFormats::Image,
            1 => GodotCtexDataFormats::Png,
            2 => GodotCtexDataFormats::Webp,
            3 => GodotCtexDataFormats::BasisUniversal,
            _ => panic!("GodotCtexFormats: Failed to convert."),
        }
    }
}



impl GodotCtex {

    pub fn load(data: impl Read) -> Result<Self, Box<dyn Error>> {
        let mut reader = ByteReader::endian(data, LittleEndian);

        assert!(reader.read::<u32>()? == 0x32545347, "GodotCtex magic check failed.");
        assert!(reader.read::<u32>()? == 1, "GodotCtex version expected to be 1.");
        let original_width: u32 = reader.read()?;
        let original_height: u32 = reader.read()?;
        let flags = GodotCtexFlags::from_bits_retain(reader.read()?);
        let original_num_mips: i32 = reader.read()?;
        reader.skip(3 * 4)?; // Reserved space

        let data_format = GodotCtexDataFormats::from(reader.read()?);
        let width: u16 = reader.read()?;
        let height: u16 = reader.read()?;
        let num_mips: u32 = reader.read()?;
        let format: u32 = reader.read()?;

        let mut mips: Vec<Vec<u8>> = Vec::new();

        match data_format {
            GodotCtexDataFormats::Png | GodotCtexDataFormats::Webp => {
                for _ in 0..=num_mips {
                    let len: u32 = reader.read()?;
                    let data = reader.read_to_vec(len as usize)?;
                    mips.push(data);
                }
            },
            GodotCtexDataFormats::Image => todo!(),
            GodotCtexDataFormats::BasisUniversal => todo!(),
        }

        Ok(GodotCtex {
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

}


