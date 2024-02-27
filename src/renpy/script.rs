
use std::{error::Error, io::{Cursor, Read, Seek}};

use crate::util::{pickle::{parser::PickleParser, pickle::Pickle}, read_ext::ReadExt};





#[derive(Debug)]
pub struct RenPyCompiledScriptChunk {
    pub slot: u32,
    pub data: Vec<u8>,
}

impl RenPyCompiledScriptChunk {
    pub fn pickle(&mut self) -> Result<Pickle, Box<dyn Error>> {
        PickleParser::parse(&mut Cursor::new(&mut self.data))
    }
}



#[derive(Debug)]
pub struct RenPyCompiledScript {
    pub chunks: Vec<RenPyCompiledScriptChunk>,
}



impl RenPyCompiledScript {

    pub fn load(data: &mut (impl Read + Seek)) -> Result<Self, Box<dyn Error>> {

        assert!(data.check_magic_string("RENPY RPC2")?, "Ren'Py script header doesn't match.");

        let mut chunks: Vec<(u32, u32, u32)> = Vec::new();
        loop {
            let slot: u32 = data.read_primitive()?;
            let offset: u32 = data.read_primitive()?;
            let length: u32 = data.read_primitive()?;
            
            if slot == 0 { break }

            chunks.push((slot, offset, length));
        }

        let chunks: Vec<RenPyCompiledScriptChunk> = chunks.iter().map(|(slot, offset, length)| {
            data.seek(std::io::SeekFrom::Start(offset.clone().into()))?;

            let compressed = data.read_to_vec(length.clone() as usize)?;
            let mut decompressed = Vec::new();
            let mut decoder = flate2::read::ZlibDecoder::new(Cursor::new(compressed));
            decoder.read_to_end(&mut decompressed)?;

            Ok(RenPyCompiledScriptChunk { slot: slot.clone(), data: decompressed })
        }).collect::<Result<Vec<_>, Box<dyn Error>>>()?;

        Ok(Self { chunks })
    }

    pub fn chunk(self, slot: u32) -> Option<RenPyCompiledScriptChunk> {
        for chunk in self.chunks {
            if chunk.slot == slot {
                return Some(chunk);
            }
        }
        None
    }

}
