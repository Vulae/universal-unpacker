
use std::{collections::HashMap, error::Error, fmt, io::{self, Cursor, Read, Seek}};
use bitflags::bitflags;
use crate::util::read_ext::ReadExt;
use super::compression::reader::GodotCompressedReader;





pub trait ReadExtResource: Read {
    fn read_real(&mut self, use_real64: bool) -> Result<f64, Box<dyn Error>>;
}

impl<T: Read> ReadExtResource for T {
    fn read_real(&mut self, use_real64: bool) -> Result<f64, Box<dyn Error>> {
        Ok(match use_real64 {
            false => self.read_primitive::<f32>()? as f64,
            true => self.read_primitive::<f64>()?,
        })
    }
}





#[derive(Debug)]
pub enum VariantObject {
    Empty,
    ExternalResource((String, String)),
    InternalResource(u32),
    ExternalResourceIndex(u32),
}

impl VariantObject {
    pub fn read(data: &mut impl Read) -> Result<Self, Box<dyn Error>> {
        Ok(match data.read_primitive::<u32>()? {
            0 => VariantObject::Empty,
            1 => VariantObject::ExternalResource((data.read_string::<u32>()?, data.read_string::<u32>()?)),
            2 => VariantObject::InternalResource(data.read_primitive()?),
            3 => VariantObject::ExternalResourceIndex(data.read_primitive()?),
            v => panic!("Unknown variant object type. \"{}\"", v),
        })
    }
}

#[derive(Debug)]
pub enum Variant {
    Nil,
    Bool(bool),
    Int(i32),
    Float(f32),
    String(String),
    Vector2((f64, f64)), // May be f32 if not use_real64
    Rect2((f64, f64, f64, f64)), // May be f32 if not use_real64
    Vector3((f64, f64, f64)), // May be f32 if not use_real64
    Plane(((f64, f64, f64), f64)), // May be f32 if not use_real64
    Quaternion((f64, f64, f64, f64)), // May be f32 if not use_real64
    AABB(((f64, f64, f64), (f64, f64, f64))), // May be f32 if not use_real64
    Transform2D(((f64, f64), (f64, f64), (f64, f64))), // May be f32 if not use_real64
    Basis(((f64, f64, f64), (f64, f64, f64), (f64, f64, f64))), // May be f32 if not use_real64
    Transform3D(((f64, f64, f64), (f64, f64, f64), (f64, f64, f64), (f64, f64, f64))), // May be f32 if not use_real64
    Color((f32, f32, f32, f32)),
    NodePath((Vec<String>, Vec<String>, bool)),
    RId(u32),
    Object(VariantObject),
    Dictionary(HashMap<String, Variant>),
    Array(Vec<Variant>),
    PackedByteArray(Vec<u8>),
    PackedInt32Array(Vec<i32>),
    PackedFloat32Array(Vec<f32>),
    PackedStringArray(Vec<String>),
    PackedVector2Array(Vec<(f64, f64)>),
    PackedVector3Array(Vec<(f64, f64, f64)>),
    PackedColorArray(Vec<(f32, f32, f32, f32)>),
    Int64(i64),
    Double(f64),
    Callable,
    Signal,
    StringName(String),
    Vector2I((i32, i32)),
    Rect2I((i32, i32, i32, i32)),
    Vector3I((i32, i32, i32)),
    PackedInt64Array(Vec<i64>),
    PackedFloat64Array(Vec<f64>),
    Vector4((f64, f64, f64, f64)), // May be f32 if not use_real64
    Vector4I((i32, i32, i32, i32)),
    Projection(((f64, f64, f64, f64), (f64, f64, f64, f64), (f64, f64, f64, f64), (f64, f64, f64, f64))), // May be f32 if not use_real64
}

impl Variant {
    pub fn read(data: &mut (impl Read + Seek), bin_version: i32, use_real64: bool) -> Result<Self, Box<dyn Error>> {
        // TODO: Implement reading all variants.
        let value = match data.read_primitive::<u32>()? {
            1 => Variant::Nil,
            2 => Variant::Bool(data.read_primitive::<u32>()? > 0),
            3 => Variant::Int(data.read_primitive()?),
            4 => Variant::Float(data.read_primitive()?),
            5 => Variant::String(data.read_string::<u32>()?),
            10 => Variant::Vector2((data.read_real(use_real64)?, data.read_real(use_real64)?)),
            12 => Variant::Vector3((data.read_real(use_real64)?, data.read_real(use_real64)?, data.read_real(use_real64)?)),
            13 => Variant::Plane(((data.read_real(use_real64)?, data.read_real(use_real64)?, data.read_real(use_real64)?), data.read_real(use_real64)?)),
            20 => Variant::Color((data.read_primitive()?, data.read_primitive()?, data.read_primitive()?, data.read_primitive()?)),
            24 => Variant::Object(VariantObject::read(data)?),
            26 => {
                let mut dict: HashMap<String, Variant> = HashMap::new();
                let mut len: u32 = data.read_primitive()?;
                len &= 0x7FFFFFFF; // Last bit set = shared.
                for _ in 0..len {
                    let key = Variant::read(data, bin_version, use_real64)?;
                    let key = match key {
                        Variant::String(str) => str,
                        _ => return Err(Box::new(ResourceError::VariantDictionaryKeyNotString)),
                    };
                    let value = Variant::read(data, bin_version, use_real64)?;
                    dict.insert(key, value);
                }
                Variant::Dictionary(dict)
            },
            30 => {
                let mut len: u32 = data.read_primitive()?;
                len &= 0x7FFFFFFF; // Last bit set = shared.
                let mut items: Vec<Variant> = Vec::new();
                for _ in 0..len {
                    items.push(Variant::read(data, bin_version, use_real64)?);
                }
                Variant::Array(items)
            },
            31 => {
                let len: u32 = data.read_primitive()?;
                let items = data.read_to_vec(len as usize)?;
                // Padding
                let extra = 4 - (len % 4);
                if extra < 4 {
                    data.seek(io::SeekFrom::Current(extra.into()))?;
                }

                Variant::PackedByteArray(items)
            },
            32 => {
                let len: u32 = data.read_primitive()?;
                let mut items: Vec<i32> = Vec::new();
                for _ in 0..len {
                    items.push(data.read_primitive()?);
                }
                Variant::PackedInt32Array(items)
            },
            33 => {
                let len: u32 = data.read_primitive()?;
                let mut items: Vec<f32> = Vec::new();
                for _ in 0..len {
                    items.push(data.read_primitive()?);
                }
                Variant::PackedFloat32Array(items)
            },
            34 => {
                let len: u32 = data.read_primitive()?;
                let mut items: Vec<String> = Vec::new();
                for _ in 0..len {
                    items.push(data.read_string::<u32>()?);
                }
                Variant::PackedStringArray(items)
            },
            36 => {
                let len: u32 = data.read_primitive()?;
                let mut items: Vec<(f32, f32, f32, f32)> = Vec::new();
                for _ in 0..len {
                    items.push((data.read_primitive()?, data.read_primitive()?, data.read_primitive()?, data.read_primitive()?));
                }
                Variant::PackedColorArray(items)
            },
            41 => Variant::Double(data.read_primitive()?),
            44 => Variant::StringName(data.read_string::<u32>()?),
            48 => {
                let len: u32 = data.read_primitive()?;
                let mut items: Vec<i64> = Vec::new();
                for _ in 0..len {
                    items.push(data.read_primitive()?);
                }
                Variant::PackedInt64Array(items)
            }
            v => return Err(Box::new(ResourceError::UnknownVariant(v))),
        };
        Ok(value)
    }
}



bitflags! {
    #[derive(Debug)]
    pub struct ResourceFlags: u32 {
        const NAMED_SCENE_IDS = 1;
        const UIDS = 2;
        const REAL_T_IS_DOUBLE = 4;
        const HAS_SCRIPT_CLASS = 8;
    }
}

impl ResourceFlags {
    const RESERVED_FIELDS: u32 = 11;
}



#[derive(Debug, Clone)]
enum ResourceError {
    UnsupportedVersion(i32), // TODO: Only version 5 is supported.
    CompressionNotSupported, // TODO: Support compressed resources.
    BigEndianNotSupported, // TODO: Support big endian
    VariantDictionaryKeyNotString,
    UnknownVariant(u32),
}

impl fmt::Display for ResourceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedVersion(version) => write!(f, "Resources of version {} not supported.", version),
            Self::CompressionNotSupported => write!(f, "Compressed resource not supported."),
            Self::BigEndianNotSupported => write!(f, "Big endian not supported."),
            Self::VariantDictionaryKeyNotString => write!(f, "Variant dictionary key must be string."),
            Self::UnknownVariant(v) => write!(f, "Unknown variant {}.", v),
        }
    }
}

impl Error for ResourceError { }



#[derive(Debug)]
pub struct ResourceContainer {
    pub version: (u32, u32),
    pub bin_version: i32,
    pub resource_type: String,
    pub flags: ResourceFlags,
    pub uid: Option<u64>,
    pub script_class: Option<String>,
    pub string_table: Vec<String>,
    pub external_resources: Vec<(String, String, Option<u64>)>,
    pub internal_resources: Vec<(String, String, Vec<(u32, Variant)>)>,
}



impl ResourceContainer {
    const IDENTIFIER_COMPRESSED: [u8; 4] = *b"RSCC";
    const IDENTIFIER_UNCOMPRESSED: [u8; 4] = *b"RSRC";

    pub fn load<D: Read + Seek>(mut data: &mut D) -> Result<Self, Box<dyn Error>> {
        let mut data = match data.read_primitive::<[u8; 4]>()? {
            Self::IDENTIFIER_COMPRESSED => {
                println!("WARNING: Compressed file will probably not parse correctly.");
                // GodotCompressedReader::open_after_ident(data)?
                // TODO: Get this to work.
                let mut reader = GodotCompressedReader::open_after_ident(&mut data, 4)?;
                let mut c: Vec<u8> = Vec::new();
                reader.read_to_end(&mut c)?;
                Cursor::new(c)
            },
            Self::IDENTIFIER_UNCOMPRESSED => {
                // reader
                let mut c: Vec<u8> = Vec::new();
                data.seek(io::SeekFrom::Start(0))?;
                data.read_to_end(&mut c)?;
                let mut data = Cursor::new(c);
                data.seek(std::io::SeekFrom::Current(4))?;
                data
            },
            _ => panic!("Resource identifier does not match."),
        };
    
        let big_endian: bool = data.read_primitive::<u32>()? > 0;
        if big_endian {
            return Err(Box::new(ResourceError::BigEndianNotSupported));
        }
    
        let use_real64: bool = data.read_primitive::<u32>()? > 0;
    
        let version: (u32, u32) = (data.read_primitive()?, data.read_primitive()?);
        let bin_version: i32 = data.read_primitive()?;
        if bin_version != 3 && bin_version != 5 {
            return Err(Box::new(ResourceError::UnsupportedVersion(bin_version)));
        }
    
        let resource_type = data.read_string::<u32>()?;

        let _metadata_offset: u64 = data.read_primitive()?;
        let flags = ResourceFlags::from_bits_retain(data.read_primitive()?);

        let uid: Option<u64> = if flags.intersects(ResourceFlags::UIDS) { Some(data.read_primitive()?) } else { data.seek(io::SeekFrom::Current(4))?; None };
        let script_class = if flags.intersects(ResourceFlags::HAS_SCRIPT_CLASS) { Some(data.read_string::<u32>()?) } else { None };
        for _ in 0..ResourceFlags::RESERVED_FIELDS {
            data.seek(io::SeekFrom::Current(4))?;
        }
    
        let mut string_table: Vec<String> = Vec::new();
        for _ in 0..data.read_primitive::<u32>()? {
            string_table.push(data.read_string::<u32>()?);
        }

        let mut external_resources: Vec<(String, String, Option<u64>)> = Vec::new();
        let mut num_external_resources: u32 = data.read_primitive()?;
        if bin_version == 3 {
            num_external_resources /= 2; // TODO: Why???
        }
        for _ in 0..num_external_resources {
            external_resources.push((
                data.read_string::<u32>()?, // Type
                data.read_string::<u32>()?, // Path
                // Uid
                if flags.intersects(ResourceFlags::UIDS) { Some(data.read_primitive()?) } else { None },
            ));
        }
    
        if bin_version == 3 {
            data.seek(io::SeekFrom::Current(4))?; // TODO: Why???
        }

        let mut internal_resources: Vec<(String, u64)> = Vec::new();
        for _ in 0..data.read_primitive::<u32>()? {
            internal_resources.push((
                data.read_string::<u32>()?, // Path
                if bin_version == 3 { data.read_primitive::<u32>()? as u64 } else { data.read_primitive()? }, // Offset
            ));
        }

        let mut parsed_internal_resources: Vec<(String, String, Vec<(u32, Variant)>)> = Vec::new();
        for internal_resource in &internal_resources {
            data.seek(std::io::SeekFrom::Start(internal_resource.1))?;
            let r#type = data.read_string::<u32>()?;
            let mut properties = Vec::new();
            for _ in 0..data.read_primitive::<u32>()? {
                let name_index: u32 = data.read_primitive()?;
                // println!("========== BEGIN VARIANT ==========");
                // println!("{} at {:#x}", name_index, reader.reader().stream_position()?);
                let variant = Variant::read(&mut data, bin_version, use_real64)?;
                properties.push((name_index, variant));
            }
            parsed_internal_resources.push((
                internal_resource.0.clone(),
                r#type,
                properties
            ));
        }
    
        Ok(ResourceContainer {
            version,
            bin_version,
            resource_type,
            flags,
            uid,
            script_class,
            string_table,
            external_resources,
            internal_resources: parsed_internal_resources,
        })
    }

}    
