
use std::{collections::HashMap, error::Error, io::{Cursor, Read, Seek}};

use crate::util::{pickle::{parser::PickleParser, pickle::{Pickle, PickleClass}}, read_ext::ReadExt};





fn ast_parse(node: Pickle) -> Result<String, Box<dyn Error>> {
    let class: PickleClass = node.clone().try_into()?;


    match class.module.module.as_str() {
        "renpy.ast" => match class.module.name.as_str() {
            "Return" => Ok(String::new()),
            "Label" => {
                let dict: HashMap<String, Pickle> = TryInto::<(Pickle, Pickle)>::try_into(*class.state.unwrap())?.1.try_into()?;
                let name: String = dict.get("name").unwrap().clone().try_into()?;
                let block: Vec<Pickle> = dict.get("block").unwrap().clone().try_into()?;
                Ok(format!(
                    "label {}:\n\t{}",
                    name,
                    block
                        .iter()
                        .rev() // Is this suppose to be reversed?
                        .map(|node| ast_parse(node.clone()))
                        .collect::<Result<Vec<_>, _>>()?
                        .join("\n\t")
                ))
            },
            "Jump" => {
                let dict: HashMap<String, Pickle> = TryInto::<(Pickle, Pickle)>::try_into(*class.state.unwrap())?.1.try_into()?;
                let target: String = dict.get("target").unwrap().clone().try_into()?;
                Ok(format!("jump {}", target))
            },
            "Say" => {
                let dict: HashMap<String, Pickle> = TryInto::<(Pickle, Pickle)>::try_into(*class.state.unwrap())?.1.try_into()?;
                let who: Option<String> = if let Ok(who) = TryInto::<String>::try_into(dict.get("who").unwrap().clone()) { Some(who) } else { None };
                let what: String = dict.get("what").unwrap().clone().try_into()?;
                Ok(if let Some(who) = who {
                    format!("{} \"{}\"", who, what)
                } else {
                    format!("\"{}\"", what)
                })
            },
            "UserStatement" => {
                let dict: HashMap<String, Pickle> = TryInto::<(Pickle, Pickle)>::try_into(*class.state.unwrap())?.1.try_into()?;
                // TODO: Use dict["parsed"].
                let line: String = dict.get("line").unwrap().clone().try_into()?;
                Ok(line)
            },
            _ => {println!("Unknown node {:?}", &node); Ok(String::new())},
        },
        _ => {println!("Unknown node {:?}", &node); Ok(String::new())},
    }
}





#[derive(Debug)]
pub struct RenPyCompiledScriptChunk {
    pub slot: u32,
    pub data: Vec<u8>,
}

impl RenPyCompiledScriptChunk {

    pub fn pickle(&mut self) -> Result<Pickle, Box<dyn Error>> {
        PickleParser::parse(&mut Cursor::new(&mut self.data))
    }

    pub fn decompile(&mut self) -> Result<String, Box<dyn Error>> {
        let pickle = self.pickle()?;

        let (header, ast): (_, Pickle) = pickle.try_into()?;
        let header: HashMap<_, _> = header.try_into()?;

        let version = if let Some(version) = header.get("version") {
            let version: i64 = version.clone().try_into()?;
            if version != 5003000 {
                panic!("Unknown script version.");
            }
            version
        } else {
            panic!("No script version.");
        };
        
        let key = if let Some(key) = header.get("key") {
            let key: String = key.clone().try_into()?;
            if key != "unlocked" {
                panic!("Unknown script key.");
            }
            key
        } else {
            panic!("No script key.");
        };

        let mut decompiled = String::new();
        decompiled += "# Decompiled Ren'Py script.\n";
        decompiled += "# Decompilation may not be accurate to source code.\n";



        let ast: Vec<Pickle> = ast.try_into()?;

        for node in ast {
            decompiled += &ast_parse(node)?;
            decompiled += "\n";
        }



        Ok(decompiled)
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
