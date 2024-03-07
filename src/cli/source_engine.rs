
use std::{error::Error, path::PathBuf};
use clap::Parser;
use crate::{extract::source_engine::vpk::{SourceEngineVpkArchive, SourceEngineVpkArchiveFiles}, util::dir_extract};



#[derive(Parser, Debug)]
pub struct CliSource {
    #[arg(index = 1)]
    file: PathBuf,
}



impl CliSource {

    fn mapper(path: String, data: &mut Vec<u8>) -> Result<Option<(String, Vec<u8>)>, Box<dyn Error>> {
        println!("File: \"{}\"", path);
        Ok(None)
    }

    pub fn extract(&self, output: &PathBuf, overwrite_output: bool) -> Result<(), Box<dyn Error>> {
        
        println!("Loading archive");

        let archive_files = SourceEngineVpkArchiveFiles::locate(&self.file)?;
        let mut archive = SourceEngineVpkArchive::from_files(archive_files)?;

        println!("Extracting archive");

        dir_extract(&mut archive, output, overwrite_output, Self::mapper)?;

        println!("Done");

        Ok(())
    }

}