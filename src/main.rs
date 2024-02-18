
mod godot;

use std::{error::Error, path::PathBuf};
use clap::{Parser, Subcommand};



#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Extract {
        #[arg(index = 1)]
        output: PathBuf,
        #[command(subcommand)]
        method: ExtractionMethods,
    },
}

#[derive(Subcommand, Debug)]
enum ExtractionMethods {
    GodotPck {
        #[arg(index = 1)]
        file: PathBuf,
    },
}



fn main() -> Result<(), Box<dyn Error>> {

    let args = Cli::parse();

    println!("{:#?}", &args);

    match &args.command {
        Commands::Extract { output, method } => {
            match &method {
                ExtractionMethods::GodotPck { file } => {
                    godot::extract(output, file)?;
                }
            }
        },
    }

    Ok(())
}


