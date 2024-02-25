
mod archive;

use std::{error::Error, fs::{self, File}, io::Write, path::PathBuf};
use clap::Parser;
use self::archive::RenPyArchive;





#[derive(Parser, Debug)]
pub struct CliRenPy {
    #[arg(index = 1)]
    file: PathBuf,
}



impl CliRenPy {

    pub fn extract(&self, output: &PathBuf, overwrite_output: bool) -> Result<(), Box<dyn Error>> {
        let archive_file = File::open(&self.file)?;

        println!("Loading archive");

        let archive_file = File::open(&self.file)?;
        let mut archive = RenPyArchive::from_file(archive_file)?;

        println!("Archive loaded with {} files", &archive.files.len());

        for file in &mut archive.files {
            println!("Extracting file \"{}\" - {} bytes", &file.path, &file.size);

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


