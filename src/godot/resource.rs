
use std::{collections::HashMap, error::Error, fmt, io::{Read, Seek}};
use bitstream_io::{ByteRead, ByteReader, Endianness, LittleEndian};
use bitflags::bitflags;



#[derive(Debug)]
pub enum VariantObject {
    Empty,
    ExternalResource((String, String)),
    InternalResource(u32),
    ExternalResourceIndex(u32),
}

impl VariantObject {
    pub fn read(data: impl Read) -> Result<Self, Box<dyn Error>> {
        let mut reader = ByteReader::endian(data, LittleEndian);
        Ok(match reader.read::<u32>()? {
            0 => VariantObject::Empty,
            1 => VariantObject::ExternalResource((reader.read_string()?, reader.read_string()?)),
            2 => VariantObject::InternalResource(reader.read()?),
            3 => VariantObject::ExternalResourceIndex(reader.read()?),
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
    pub fn read(data: impl Read, use_real64: bool) -> Result<Self, Box<dyn Error>> {
        let mut reader = ByteReader::endian(data, LittleEndian);
        // TODO: Implement reading all variants.
        let value = match reader.read::<u32>()? {
            1 => Variant::Nil,
            2 => Variant::Bool(reader.read::<u32>()? > 0),
            3 => Variant::Int(reader.read()?),
            4 => Variant::Float(reader.read()?),
            5 => Variant::String(reader.read_string()?),
            10 => Variant::Vector2((reader.read_real(use_real64)?, reader.read_real(use_real64)?)),
            12 => Variant::Vector3((reader.read_real(use_real64)?, reader.read_real(use_real64)?, reader.read_real(use_real64)?)),
            13 => Variant::Plane(((reader.read_real(use_real64)?, reader.read_real(use_real64)?, reader.read_real(use_real64)?), reader.read_real(use_real64)?)),
            20 => Variant::Color((reader.read()?, reader.read()?, reader.read()?, reader.read()?)),
            24 => Variant::Object(VariantObject::read(reader.reader())?),
            26 => {
                let mut dict: HashMap<String, Variant> = HashMap::new();
                let mut len: u32 = reader.read()?;
                len &= 0x7FFFFFFF; // Last bit set = shared.
                for _ in 0..len {
                    let key = Variant::read(reader.reader_ref(), use_real64)?;
                    let key = match key {
                        Variant::String(str) => str,
                        _ => return Err(Box::new(ResourceError::VariantDictionaryKeyNotString)),
                    };
                    let value = Variant::read(reader.reader_ref(), use_real64)?;
                    dict.insert(key, value);
                }
                Variant::Dictionary(dict)
            },
            30 => {
                let mut len: u32 = reader.read()?;
                len &= 0x7FFFFFFF; // Last bit set = shared.
                let mut items: Vec<Variant> = Vec::new();
                for _ in 0..len {
                    items.push(Variant::read(reader.reader_ref(), use_real64)?);
                }
                Variant::Array(items)
            },
            31 => {
                let len: u32 = reader.read()?;
                let items = reader.read_to_vec(len as usize)?;
                // Padding
                let extra = 4 - (len % 4);
                if extra < 4 { reader.skip(extra)?; }

                Variant::PackedByteArray(items)
            },
            32 => {
                let len: u32 = reader.read()?;
                let mut items: Vec<i32> = Vec::new();
                for _ in 0..len {
                    items.push(reader.read()?);
                }
                Variant::PackedInt32Array(items)
            },
            33 => {
                let len: u32 = reader.read()?;
                let mut items: Vec<f32> = Vec::new();
                for _ in 0..len {
                    items.push(reader.read()?);
                }
                Variant::PackedFloat32Array(items)
            },
            34 => {
                let len: u32 = reader.read()?;
                let mut items: Vec<String> = Vec::new();
                for _ in 0..len {
                    items.push(reader.read_string()?);
                }
                Variant::PackedStringArray(items)
            },
            36 => {
                let len: u32 = reader.read()?;
                let mut items: Vec<(f32, f32, f32, f32)> = Vec::new();
                for _ in 0..len {
                    items.push((reader.read()?, reader.read()?, reader.read()?, reader.read()?));
                }
                Variant::PackedColorArray(items)
            },
            41 => Variant::Double(reader.read()?),
            44 => Variant::StringName(reader.read_string()?),
            48 => {
                let len: u32 = reader.read()?;
                let mut items: Vec<i64> = Vec::new();
                for _ in 0..len {
                    items.push(reader.read()?);
                }
                Variant::PackedInt64Array(items)
            }
            v => {
                panic!("Unknown variant type. \"{}\"", v);
            },
        };
        Ok(value)
    }
}



trait ByteReaderExtend {
    fn read_string(&mut self) -> Result<String, Box<dyn Error>>;
    fn read_real(&mut self, use_real64: bool) -> Result<f64, Box<dyn Error>>;
}

impl<R: Read, E: Endianness> ByteReaderExtend for ByteReader<R, E> {
    fn read_string(&mut self) -> Result<String, Box<dyn Error>> {
        let len: u32 = self.read()?;
        let str = String::from_utf8(self.read_to_vec(len as usize)?)?.trim_end_matches("\0").to_owned();
        // let str = String::from_utf8(self.read_to_vec(len as usize)?)?;
        Ok(str)
    }

    fn read_real(&mut self, use_real64: bool) -> Result<f64, Box<dyn Error>> {
        match use_real64 {
            false => Ok(self.read::<f32>()? as f64),
            true => Ok(self.read()?)
        }
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
    UnsupportedVersion, // TODO: Only version 5 is supported.
    CompressionNotSupported, // TODO: Support compressed resources.
    BigEndianNotSupported, // TODO: Support big endian
    VariantDictionaryKeyNotString,
}

impl fmt::Display for ResourceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedVersion => write!(f, "Resources of version 5 only supported."),
            Self::CompressionNotSupported => write!(f, "Compressed resource not supported."),
            Self::BigEndianNotSupported => write!(f, "Big endian not supported."),
            Self::VariantDictionaryKeyNotString => write!(f, "Variant dictionary key must be string."),
        }
    }
}

impl Error for ResourceError { }



#[derive(Debug)]
pub struct ResourceContainer {
    pub version: (u32, u32),
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

    pub fn load(data: impl Read + Seek) -> Result<Self, Box<dyn Error>> {
        let mut reader = ByteReader::endian(data, LittleEndian);
    
        let mut reader = match reader.read::<[u8; 4]>()? {
            // TODO: Mutate stream to be compressed reader.
            Self::IDENTIFIER_COMPRESSED => return Err(Box::new(ResourceError::CompressionNotSupported)),
            Self::IDENTIFIER_UNCOMPRESSED => reader,
            _ => panic!("Resource identifier does not match."),
        };
    
        let big_endian: bool = reader.read::<u32>()? > 0;
        if big_endian {
            return Err(Box::new(ResourceError::BigEndianNotSupported));
        }
    
        let use_real64: bool = reader.read::<u32>()? > 0;
    
        let version: (u32, u32) = (reader.read()?, reader.read()?);
        let resource_format_version: i32 = reader.read()?;
        if resource_format_version != 5 {
            return Err(Box::new(ResourceError::UnsupportedVersion));
        }
    
        let resource_type = reader.read_string()?;
    
        let _metadata_offset: u64 = reader.read()?;
        let flags = ResourceFlags::from_bits_retain(reader.read()?);

        let uid: Option<u64> = if flags.intersects(ResourceFlags::UIDS) { Some(reader.read()?) } else { reader.skip(4)?; None };
        let script_class = if flags.intersects(ResourceFlags::HAS_SCRIPT_CLASS) { Some(reader.read_string()?) } else { None };
        for _ in 0..ResourceFlags::RESERVED_FIELDS {
            reader.skip(4)?;
        }
    
        let mut string_table: Vec<String> = Vec::new();
        for _ in 0..reader.read::<u32>()? {
            string_table.push(reader.read_string()?);
        }
    
        let mut external_resources: Vec<(String, String, Option<u64>)> = Vec::new();
        for _ in 0..reader.read::<u32>()? {
            external_resources.push((
                reader.read_string()?, // Type
                reader.read_string()?, // Path
                // Uid
                if flags.intersects(ResourceFlags::UIDS) { Some(reader.read()?) } else { None },
            ));
        }
    
        let mut internal_resources: Vec<(String, u64)> = Vec::new();
        for _ in 0..reader.read::<u32>()? {
            internal_resources.push((
                reader.read_string()?, // Path
                reader.read::<u64>()?, // Offset
            ));
        }

        let mut parsed_internal_resources: Vec<(String, String, Vec<(u32, Variant)>)> = Vec::new();
        for internal_resource in &internal_resources {
            reader.reader().seek(std::io::SeekFrom::Start(internal_resource.1))?;
            let r#type = reader.read_string()?;
            let mut properties = Vec::new();
            for _ in 0..reader.read::<u32>()? {
                let name_index: u32 = reader.read()?;
                // println!("========== BEGIN VARIANT ==========");
                // println!("{} at {:#x}", name_index, reader.reader().stream_position()?);
                let variant = Variant::read(reader.reader(), use_real64)?;
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
