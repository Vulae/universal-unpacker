
use std::{error::Error, fs::{self, File}, io::Write, path::PathBuf};
use clap::Parser;
use crate::godot::archive::GodotPck;

mod archive;



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
}



impl CliGodotPck {
    
    pub fn extract(&self, output: &PathBuf, overwrite_output: bool) -> Result<(), Box<dyn Error>> {
        let key: Option<[u8; 32]> = if let Some(key) = &self.key {
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

            let mut output_file_path = PathBuf::from(output);
            output_file_path.push(&file.path.trim_start_matches("res://"));

            if let Ok(meta) = fs::metadata(&output_file_path) {
                if meta.is_file() && !overwrite_output {
                    println!("Skipped \"{}\"", &file.path);
                    continue;
                }
            }

            println!("Extracting file \"{}\" - {} bytes", &file.path, &file.size);
            fs::create_dir_all(output_file_path.parent().unwrap())?;
    
            let mut data = file.read_data(key)?;

            let mut output_file = File::create(output_file_path)?;
            output_file.write_all(&mut data)?;
            output_file.flush()?;
        }

        println!("Done extracting archive");

        Ok(())
    }

}


