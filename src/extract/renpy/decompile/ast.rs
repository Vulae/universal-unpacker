
use std::{collections::HashMap, error::Error};
use crate::util::pickle::pickle::{Pickle, PickleClass};
use super::parse_node;



pub fn parse_node_ast(node: Pickle) -> Result<String, Box<dyn Error>> {
    let class: PickleClass = node.clone().try_into()?;

    match class.module.name.as_str() {
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
                    .map(|node| parse_node(node.clone()))
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
        class => { println!("Unknown class. {}", class); Ok(String::new()) },
    }
}
