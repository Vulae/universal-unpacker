
use std::{error::Error, fs::File, io::Cursor, path::PathBuf};
use clap::Parser;
use crate::{extract::source_engine::{source1::vtf::VTF, vpk::{SourceEngineVpkArchive, SourceEngineVpkArchiveFiles}}, util::dir_extract};



#[derive(Parser, Debug)]
pub struct CliSource {
    #[arg(index = 1)]
    file: PathBuf,
}



impl CliSource {

    fn mapper(path: String, data: &mut Vec<u8>) -> Result<Option<(String, Vec<u8>)>, Box<dyn Error>> {
        println!("File: \"{}\"", path);

        if path.ends_with(".vtf") {
            let vtf = VTF::load(&mut Cursor::new(data))?;
        }

        Ok(None)
    }

    pub fn extract(&self, output: &PathBuf, overwrite_output: bool) -> Result<(), Box<dyn Error>> {
        if let Some(ext) = self.file.extension() {
            match ext.to_str() {
                Some("vpk") => {
                    println!("Loading archive");

                    let archive_files = SourceEngineVpkArchiveFiles::locate(&self.file)?;
                    let mut archive = SourceEngineVpkArchive::from_files(archive_files)?;
            
                    println!("Extracting archive");
            
                    dir_extract(&mut archive, output, overwrite_output, Self::mapper)?;
            
                    println!("Done");
                },
                Some("vtf") => {
                    println!("Loading VTF");
                    let vtf = VTF::load(File::open(&self.file)?)?;
                    if let Some(texture) = vtf.texture(0, 0, 0, 0) {
                        println!("Converting VTF");
                        let image = texture.to_image();
                        image.save(output)?;
                    }
                    println!("Done");
                },
                _ => { },
            }
        }

        Ok(())
    }

}