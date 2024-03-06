
use std::{error::Error, fs::{self, File}, io::Write, path::PathBuf};
use clap::Parser;
use crate::{extract::source_engine::vpk::{SourceEngineVpkArchive, SourceEngineVpkArchiveFiles}, util::virtual_fs::{VirtualDirectory, VirtualEntry, VirtualFile}};



#[derive(Parser, Debug)]
pub struct CliSource {
    #[arg(index = 1)]
    file: PathBuf,
}



pub fn dir_extract<'a, F, D>(mut entry: VirtualEntry<'a, F, D>, output: &PathBuf, overwrite_output: bool) -> Result<(), Box<dyn Error>>
where
    F: VirtualFile,
    D: VirtualDirectory<F, D>
{
    let mut path = PathBuf::from(output);
    path.push(entry.path());
    match entry {
        VirtualEntry::File(file) => {
            println!("File: {:?}", path);

            if let Ok(meta) = fs::metadata(&path) {
                if meta.is_file() && !overwrite_output {
                    return Ok(());
                }
            }

            fs::create_dir_all(path.parent().unwrap())?;

            let mut output_file = File::create(path)?;
            let mut data = file.read_data()?;
            output_file.write_all(&mut data)?;
            output_file.flush()?;

            Ok(())
        },
        VirtualEntry::Directory(dir) => {
            for entry in dir.read_entries()? {
                dir_extract(entry, &path, overwrite_output)?;
            }
            Ok(())
        },
    }
}



impl CliSource {

    pub fn extract(&self, output: &PathBuf, overwrite_output: bool) -> Result<(), Box<dyn Error>> {
        
        println!("Loading archive");

        let archive_files = SourceEngineVpkArchiveFiles::locate(&self.file)?;
        let mut archive = SourceEngineVpkArchive::from_files(archive_files)?;

        println!("Extracting archive");

        dir_extract(VirtualEntry::Directory(&mut archive), output, overwrite_output)?;

        println!("Done");

        Ok(())
    }

}