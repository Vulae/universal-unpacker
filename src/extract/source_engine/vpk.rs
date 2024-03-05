
use std::{error::Error, fs::{self, File}, io::{Seek, SeekFrom}, path::PathBuf};
use regex::Regex;

use crate::util::read_ext::ReadExt;



/// 
/// A container to contain all pak files for the specific archive.
/// 
#[derive(Debug, Clone)]
pub struct SourceEngineVpkArchiveFiles {
    /// The directory with file list, may contain file content.
    pub dir: PathBuf,
    /// Directories without file list, always have file content.
    pub entries: Vec<PathBuf>,
}

impl SourceEngineVpkArchiveFiles {

    pub fn locate(path: &PathBuf) -> Result<Self, Box<dyn Error>> {
        if !path.is_file() {
            panic!("Path must be file.");
        }

        let path_filename = path.file_name().unwrap().to_str().unwrap();
        let path_filename_regex = Regex::new(r"(.+?)(?:_dir|_\d+)?.vpk")?;

        if let Some(caps) = path_filename_regex.captures(path_filename) {
            let archive_name = caps.get(1).unwrap().as_str();

            let mut dir: Option<PathBuf> = None;
            let mut entries: Vec<PathBuf> = Vec::new();
            
            for entry in fs::read_dir(path.parent().unwrap())? {
                let entry = entry.unwrap();
                if entry.path().is_dir() { continue; }

                let filename = entry.file_name();
                let filename = filename.to_str().unwrap();
                let filename_regex = Regex::new(r"(.+?)(?:_(dir|\d+))?\.vpk")?;

                if let Some(caps) = filename_regex.captures(filename) {
                    if caps.get(1).unwrap().as_str() != archive_name {
                        continue;
                    }

                    match caps.get(2) {
                        Some(cap) => {
                            if cap.as_str() == "dir" {
                                dir = Some(entry.path());
                            } else {
                                entries.push(entry.path());
                            }
                        },
                        None => {
                            dir = Some(entry.path())
                        }
                    }
                }
            }

            entries.sort();

            if let Some(dir) = dir {
                return Ok(Self { dir, entries });
            }

        }

        panic!("Invalid VPK archive.");

    }

}





#[derive(Debug)]
pub struct SourceEngineVPKFile {
    archive_file: File,
    pub path: String,
    offset: u32,
    pub length: u32,
}

impl SourceEngineVPKFile {

    // TODO: Checksum.
    pub fn read_data(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        self.archive_file.seek(SeekFrom::Start(self.offset as u64))?;
        Ok(self.archive_file.read_to_vec(self.length as usize)?)
    }

}

#[derive(Debug)]
pub struct SourceEngineVpkArchive {
    pub files: Vec<SourceEngineVPKFile>,
}

impl SourceEngineVpkArchive {

    pub fn from_files(archive_files: SourceEngineVpkArchiveFiles) -> Result<Self, Box<dyn Error>> {
        let mut archive_dir = File::open(archive_files.dir)?;
        let archive_entries = archive_files.entries.iter().map(|entry| File::open(entry)).collect::<Result<Vec<_>, _>>()?;

        assert!(archive_dir.check_magic::<u32>(0x55AA1234)?);
        let version = archive_dir.read_primitive::<u32>()?;
        let tree_size = archive_dir.read_primitive::<u32>()?;

        match version {
            1 => { },
            2 => { archive_dir.seek(SeekFrom::Current(16))?; },
            version => panic!("Unsupported version: {}", version),
        }

        let end_of_dir = archive_dir.stream_position()? + (tree_size as u64);

        let mut files = Vec::new();

        loop {
            let ext = archive_dir.read_terminated_string(0x00)?;
            if ext.is_empty() { break; }
            loop {
                let path = archive_dir.read_terminated_string(0x00)?;
                if path.is_empty() { break; }
                loop {
                    let name = archive_dir.read_terminated_string(0x00)?;
                    if name.is_empty() { break; }

                    let crc = archive_dir.read_primitive::<u32>()?;
                    let preload = archive_dir.read_primitive::<u16>()?;
                    let archive_index = archive_dir.read_primitive::<u16>()?;
                    let offset = archive_dir.read_primitive::<u32>()?;
                    let length = archive_dir.read_primitive::<u32>()?;

                    assert!(archive_dir.check_magic::<u16>(0xFFFF)?);

                    if preload > 0 {
                        println!("Preload files not supported.");
                        continue;
                    }

                    files.push(SourceEngineVPKFile {
                        archive_file: (if archive_index == 0x7FFF {
                            archive_dir.try_clone()?
                        } else {
                            archive_entries[archive_index as usize].try_clone()?
                        }),
                        path: if path.trim().is_empty() { format!("{}.{}", name, ext) } else { format!("{}/{}.{}", path, name, ext) },
                        offset: (if archive_index == 0x7FFF { offset + (end_of_dir as u32) } else { offset }),
                        length
                    });
                }
            }
        }

        Ok(Self { files })
    }

}


