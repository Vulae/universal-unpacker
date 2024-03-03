
use std::error::Error;
use crate::util::pickle::pickle::{Pickle, PickleClass};
use super::{indent, node_extract_data, parse_node};



pub fn parse_node_ast(node: Pickle) -> Result<String, Box<dyn Error>> {
    let class: PickleClass = node.clone().try_into()?;

    Ok(match class.module.name.as_str() {
        "Return" => String::new(),
        "Label" => format!(
            "label {}:\n{}\n",
            TryInto::<String>::try_into(node_extract_data(&node, "name")?)?,
            indent(TryInto::<Vec<Pickle>>::try_into(node_extract_data(&node, "block")?)?
                .iter()
                .map(|node| parse_node(node.clone()))
                .collect::<Result<Vec<_>, _>>()?
                .join(""))
        ),
        "Jump" => format!(
            "jump {}\n",
            TryInto::<String>::try_into(node_extract_data(&node, "target")?)?
        ),
        "Say" => {
            let what: String = TryInto::<String>::try_into(node_extract_data(&node, "what")?)?;
            if let Some(who) = TryInto::<Option<String>>::try_into(node_extract_data(&node, "who")?)? {
                format!("{} \"{}\"\n", who, what)
            } else {
                format!("\"{}\"\n", what)
            }
        },
        "UserStatement" => {
            // TODO: Use "parsed".
            format!("{}\n", TryInto::<String>::try_into(node_extract_data(&node, "line")?)?)
        },
        "With" => {
            // FIXME: Often gets put on a newline when should be on the same line.
            match node_extract_data(&node, "expr")? {
                Pickle::String(str) => {
                    if str == "None" {
                        String::new()
                    } else {
                        format!("with {}\n", str)
                    }
                },
                Pickle::Class(class) => format!("with {}\n", parse_node(Pickle::Class(class))?),
                _ => panic!(),
            }
        },
        "PyExpr" => {
            let args = TryInto::<(Pickle, Pickle, Pickle, Pickle)>::try_into(*class.args)?;
            format!("{}", TryInto::<String>::try_into(args.3)?)
        },
        "Scene" => {
            let imspec = TryInto::<(Pickle, Pickle, Pickle, Pickle, Pickle, Pickle, Pickle)>::try_into(node_extract_data(&node, "imspec")?)?;
            format!(
                "scene {}\n",
                TryInto::<Vec<Pickle>>::try_into(imspec.6)?
                    .iter()
                    .map(|item| TryInto::<String>::try_into(item.clone()))
                    .collect::<Result<Vec<_>, _>>()?
                    .join(" ")
            )
        },
        "Show" => {
            let imspec = TryInto::<(Pickle, Pickle, Pickle, Pickle, Pickle, Pickle, Pickle)>::try_into(node_extract_data(&node, "imspec")?)?;
            format!(
                "show {}\n",
                TryInto::<Vec<Pickle>>::try_into(imspec.6)?
                    .iter()
                    .map(|item| TryInto::<String>::try_into(item.clone()))
                    .collect::<Result<Vec<_>, _>>()?
                    .join(" ")
            )
        },
        "Hide" => {
            let imspec = TryInto::<(Pickle, Pickle, Pickle, Pickle, Pickle, Pickle, Pickle)>::try_into(node_extract_data(&node, "imspec")?)?;
            format!(
                "hide {}\n",
                TryInto::<Vec<Pickle>>::try_into(imspec.6)?
                    .iter()
                    .map(|item| TryInto::<String>::try_into(item.clone()))
                    .collect::<Result<Vec<_>, _>>()?
                    .join(" ")
            )
        },
        "Init" => format!(
            "{}\n",
            TryInto::<Vec<Pickle>>::try_into(node_extract_data(&node, "block")?)?
                .iter()
                .map(|node| parse_node(node.clone()))
                .collect::<Result<Vec<_>, _>>()?
                .join("\n")
        ),
        "Define" => {
            format!(
                "define {}.{} {} {}\n",
                TryInto::<String>::try_into(node_extract_data(&node, "store")?)?,
                TryInto::<String>::try_into(node_extract_data(&node, "varname")?)?,
                TryInto::<String>::try_into(node_extract_data(&node, "operator")?)?,
                parse_node(node_extract_data(&node, "code")?)?,
            )
        },
        "Default" => {
            format!(
                "default {}.{} = {}\n",
                TryInto::<String>::try_into(node_extract_data(&node, "store")?)?,
                TryInto::<String>::try_into(node_extract_data(&node, "varname")?)?,
                parse_node(node_extract_data(&node, "code")?)?,
            )
        },
        "Python" => {
            let code = parse_node(node_extract_data(&node, "code")?)?;
            
            if code.split("\n").count() == 1 {
                format!("$ {}\n", code.replace("\n", ""))
            } else {
                format!("init python:\n{}\n", indent(code))
            }
        },
        class => format!("***DECOMPILE ERROR: Unknown class. {}***", class),
    })
}
