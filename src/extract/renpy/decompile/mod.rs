
/*
    Decompilation is not really what's happening.
    We are just taking an abstract syntax tree and putting it back into code.
*/

use std::{collections::HashMap, error::Error};
use crate::util::pickle::pickle::{Pickle, PickleClass};
use self::ast::parse_node_ast;
use super::script::RenPyCompiledScriptChunk;

mod ast;



pub fn node_extract_data(node: &Pickle, key: &str) -> Result<Pickle, Box<dyn Error>> {
    let class: PickleClass = node.clone().try_into()?;
    let dict: HashMap<String, Pickle> = TryInto::<(Pickle, Pickle)>::try_into(*class.state.unwrap())?.1.try_into()?;
    if let Some(item) = dict.get(key) {
        Ok(item.clone())
    } else {
        panic!("Unable to extract node pickle data.")
    }
}



pub fn parse_node(node: Pickle) -> Result<String, Box<dyn Error>> {
    let class: PickleClass = node.clone().try_into()?;

    Ok(match class.module.module.as_str() {
        "renpy.ast" => parse_node_ast(node)?,
        module => format!("***DECOMPILE ERROR: Unknown module. {}***", module),
    })
}

pub fn indent(str: String) -> String {
    str.split("\n").map(|str| format!("\t{}", str)).collect::<Vec<_>>().join("\n")
}





impl RenPyCompiledScriptChunk {

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
            decompiled += &parse_node(node)?;
            decompiled += "\n";
        }



        Ok(decompiled)
    }

}
