
mod godot;
mod util;

use std::{error::Error, fs::{self, File}, io::Write, path::PathBuf};
use clap::{Parser, Subcommand, ValueEnum};
use util::decode_hex;



#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(ValueEnum, Parser, Debug, Clone, Copy, PartialEq)]
enum ExtractOptions {
    /// Extract archive while keeping old files.
    None,
    /// Clean up extraction directory before extracting.
    Clean,
    /// Overwrite existing files.
    Overwrite,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Extract {
        #[arg(index = 1)]
        output: PathBuf,
        #[command(subcommand)]
        method: ExtractionMethods,
        #[arg(short, long, value_enum, default_value_t = ExtractOptions::Overwrite)]
        extract_options: ExtractOptions,
    },
}

#[derive(Subcommand, Debug)]
enum ExtractionMethods {
    GodotPck {
        #[arg(index = 1)]
        file: PathBuf,
        #[arg(short, long)]
        /// Encryption key used to decrypt archive.
        /// 
        /// [Godot encryption key](https://docs.godotengine.org/en/stable/contributing/development/compiling/compiling_with_script_encryption_key.html)
        /// 
        /// [Extract encryption key](https://github.com/pozm/gdke/tree/master)
        key: Option<String>,
    },
}

impl ExtractionMethods {
    fn extract(&mut self, output: &PathBuf, overwrite_output: bool) -> Result<(), Box<dyn Error>> {
        match self {

            Self::GodotPck { file, key } => {

                let key: Option<[u8; 32]> = if let Some(key) = key {
                    todo!("Decode hex key");
                    // Some(decode_hex(key)?.try_into().unwrap())
                } else {
                    None
                };

                println!("Loading archive");

                let archive_file = File::open(file)?;
                let mut archive = godot::GodotPck::from_file(archive_file)?;

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
                
            }

        }
        Ok(())
    }
}



fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    match args.command {
        Commands::Extract { output, mut method, extract_options } => {
            if extract_options == ExtractOptions::Clean {
                fs::remove_dir_all(&output)?;
            }

            method.extract(&output, extract_options == ExtractOptions::Overwrite)?;
        },
    }

    Ok(())
}


