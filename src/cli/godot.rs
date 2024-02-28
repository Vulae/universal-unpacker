#![allow(dead_code)]

use std::{error::Error, fs::{self, File}, io::{Cursor, Write}, path::PathBuf};
use clap::Parser;
use crate::extract::godot::{archive::GodotPck, resource::ResourceContainer, texture::Texture};



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





fn resource_format_convert(path: &String, data: &Vec<u8>) -> Option<(String, Vec<u8>)> {
    if data.len() < 4 { return None }
    match &data[0..4] {
        b"RSRC" | b"RSCC" => {
            return match ResourceContainer::load(&mut Cursor::new(data)) {
                Ok(extracted_resource) => Some(("extracted_resource".to_owned(), format!("{:#?}", extracted_resource).into_bytes())),
                Err(err) => {
                    println!("Resource parse failed \"{}\" {:#?}", path, err);
                    None
                },
            };
        },
        [b'G', b'D', _, _] | [b'G', b'S', _, _] => {
            if let Ok(mut texture) = Texture::load(Cursor::new(data)) {
                if let Ok((new_ext, image)) = texture.to_image() {
                    return Some((new_ext.to_owned(), image));
                }
            }
        },
        _ => { },
    }
    None
}



impl CliGodotPck {

    pub fn extract(&self, output: &PathBuf, overwrite_output: bool) -> Result<(), Box<dyn Error>> {
        let key: Option<[u8; 32]> = if let Some(_key) = &self.key {
            todo!("Decode hex key");
            // Some(decode_hex(key)?.try_into().unwrap())
        } else {
            None
        };

        println!("Loading archive");

        let archive_file = File::open(&self.file)?;
        let mut archive = GodotPck::from_file(archive_file)?;

        println!("Archive loaded with {} files", &archive.num_files);

        for file in &mut archive.files {
            // println!("Extracting file \"{}\" - {} bytes", &file.path, &file.size);

            let mut output_file_path = PathBuf::from(output);
            output_file_path.push(&file.path.trim_start_matches("res://"));
            
            let mut data = file.read_data(key)?;

            if self.parse {
                if let Some((new_ext, new_data)) = resource_format_convert(&file.path, &data) {
                    output_file_path.set_extension(new_ext);
                    data = new_data;
                }
            }

            if let Ok(meta) = fs::metadata(&output_file_path) {
                if meta.is_file() && !overwrite_output {
                    continue;
                }
            }

            fs::create_dir_all(output_file_path.parent().unwrap())?;

            let mut output_file = File::create(output_file_path)?;
            output_file.write_all(&mut data)?;
            output_file.flush()?;
        }

        println!("Done extracting archive");

        Ok(())
    }

}


