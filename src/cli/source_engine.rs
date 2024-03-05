
use std::{error::Error, fs::{self, File}, io::Write, path::PathBuf};
use clap::Parser;
use crate::extract::source_engine::vpk::{SourceEngineVpkArchive, SourceEngineVpkArchiveFiles};



#[derive(Parser, Debug)]
pub struct CliSource {
    #[arg(index = 1)]
    file: PathBuf,
}



impl CliSource {

    pub fn extract(&self, output: &PathBuf, overwrite_output: bool) -> Result<(), Box<dyn Error>> {
        
        println!("Loading archive");

        let archive_files = SourceEngineVpkArchiveFiles::locate(&self.file)?;
        let mut archive = SourceEngineVpkArchive::from_files(archive_files)?;

        println!("Archive loaded with {} files", &archive.files.len());

        for file in &mut archive.files {
            println!("Extracting file \"{}\" - {} bytes", &file.path, &file.length);

            let mut output_file_path = PathBuf::from(output);
            output_file_path.push(&file.path);
            
            let mut data = file.read_data()?;

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