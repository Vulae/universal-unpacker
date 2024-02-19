
mod godot;
mod util;

use std::{error::Error, fs, path::PathBuf};
use clap::{Parser, Subcommand, ValueEnum};
use godot::CliGodotPck;



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
    GodotPck(CliGodotPck),
}

impl ExtractionMethods {
    fn extract(&mut self, output: &PathBuf, overwrite_output: bool) -> Result<(), Box<dyn Error>> {
        match self {
            ExtractionMethods::GodotPck(method) => method.extract(output, overwrite_output)?,
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


