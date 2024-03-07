
use std::{error::Error, fs::File, io::Cursor, path::PathBuf};
use clap::Parser;

use crate::{extract::renpy::{archive::RenPyArchive, script::RenPyCompiledScript}, util::dir_extract};





#[derive(Parser, Debug)]
pub struct CliRenPy {
    #[arg(index = 1)]
    file: PathBuf,
}





impl CliRenPy {

    fn mapper(path: String, data: &mut Vec<u8>) -> Result<Option<(String, Vec<u8>)>, Box<dyn Error>> {
        println!("File: \"{}\"", path);

        if path.ends_with(".rpyc") {
            let script = RenPyCompiledScript::load(&mut Cursor::new(data))?;
            if let Some(mut chunk) = script.chunk(1) {
                if let Ok(str) = chunk.decompile() {
                    return Ok(Some((
                        path.replace(".rpyc", ".rpyc-decomp"),
                        str.as_bytes().to_vec()
                    )));
                } else {
                    println!("Ren'Py script decompilation error could not gracefully handle.");
                    // So we output pickle instead with message.
                    return Ok(Some((
                        path.replace(".rpyc", ".rypc-pickle"),
                        format!("CATASTROPHIC ERROR\nFile could not decompile\nPlease create a bug report with this file.\n{:#?}", chunk.pickle()?).as_bytes().to_vec()
                    )))
                }
            }
        }

        Ok(None)
    }

    pub fn extract(&self, output: &PathBuf, overwrite_output: bool) -> Result<(), Box<dyn Error>> {

        println!("Loading archive");

        let archive_file = File::open(&self.file)?;
        let mut archive = RenPyArchive::from_file(archive_file)?;

        println!("Extracting archive");

        dir_extract(&mut archive, output, overwrite_output, Self::mapper)?;

        println!("Done");

        Ok(())
    }

}


