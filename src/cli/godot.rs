#![allow(dead_code)]

use std::{error::Error, fs::File, io::Cursor, path::PathBuf};
use clap::Parser;
use crate::{extract::godot::{archive::GodotPck, resource::ResourceContainer, texture::Texture}, util::dir_extract};



#[derive(Parser, Debug)]
pub struct CliGodotPck {
    #[arg(index = 1)]
    file: PathBuf,
    #[arg(short, long)]
    /// Encryption key used to decrypt archive.
    /// 
    /// [Godot encryption key](https://docs.godotengine.org/en/stable/contributing/development/compiling/compiling_with_script_encryption_key.html)
    /// 
    /// [Extract encryption key](https://github.com/pozm/gdke/tree/master)
    key: Option<String>,
    #[arg(short, long, default_value_t = false)]
    /// If to convert compatible formats.
    /// > WARNING: Experimental, file size may VERY large!
    parse: bool,
}



impl CliGodotPck {

    fn mapper(&self, path: String, data: &mut Vec<u8>) -> Result<Option<(String, Vec<u8>)>, Box<dyn Error>> {
        println!("File: \"{}\"", path);

        let path = path.replace("res://", "");

        if self.parse {
            match &data[0..4] {
                b"RSRC" | b"RSCC" => {
                    match ResourceContainer::load(&mut Cursor::new(&data)) {
                        Ok(extracted_resource) => return Ok(Some((
                            [path, "extracted-resource".to_owned()].join("."),
                            format!("{:#?}", extracted_resource).into_bytes()
                        ))),
                        Err(err) => {
                            println!("Resource parse failed \"{}\" {:#?}", path, err);
                        },
                    };
                },
                [b'G', b'D', _, _] | [b'G', b'S', _, _] => {
                    if let Ok(mut texture) = Texture::load(Cursor::new(&data)) {
                        if let Ok((new_ext, image)) = texture.to_image() {
                            return Ok(Some((
                                [path, new_ext.to_owned()].join("."),
                                image
                            )));
                        }
                    }
                },
                _ => { },
            }
        }

        Ok(Some((path, data.clone())))
    }

    pub fn extract(&self, output: &PathBuf, overwrite_output: bool) -> Result<(), Box<dyn Error>> {
        let key: Option<[u8; 32]> = if let Some(_key) = &self.key {
            todo!("Decode hex key");
            // Some(decode_hex(key)?.try_into().unwrap())
        } else {
            None
        };

        println!("Loading archive");

        let archive_file = File::open(&self.file)?;
        let mut archive = GodotPck::from_file(archive_file, None)?;

        println!("Extracting archive");

        dir_extract(&mut archive, output, overwrite_output, |path, data| {
            self.mapper(path, data)
        })?;

        println!("Done");

        Ok(())
    }

}


