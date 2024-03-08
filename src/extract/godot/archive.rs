
use std::{error::Error, fs::File, io::{Read, Seek}};
use bitflags::bitflags;
use crate::util::{read_ext::ReadExt, virtual_fs::{VirtualDirectory, VirtualEntry, VirtualFile}};



bitflags! {
    #[derive(Debug)]
    pub struct GodotPckFileFlags: u32 {
        const ENCRYPTED_FILE = 1 << 0;
    }

    #[derive(Debug)]
    pub struct GodotPckFlags: u32 {
        const ENCRYPTED_ARCHIVE = 1 << 0;
    }
}



#[derive(Debug)]
pub struct GodotPckFile {
    file: File,
    path: String,
    offset: i64,
    size: i64,
    md5: [u8; 16],
    flags: GodotPckFileFlags,
    encryption_key: Option<[u8; 32]>,
}

impl VirtualFile for GodotPckFile {
    fn path(&mut self) -> &str {
        &self.path
    }

    fn read_data(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut buf: Vec<u8> = vec![0; self.size as usize];
        self.file.seek(std::io::SeekFrom::Start(self.offset as u64))?;
        self.file.read(&mut buf)?;

        if self.flags.contains(GodotPckFileFlags::ENCRYPTED_FILE) {
            if let Some(_encryption_key) = self.encryption_key {
                todo!("GodotPckFile decryption not yet supported.");
            } else {
                panic!("GodotPckFile tried to decrypt file that is encrypted");
            }
        }

        Ok(buf)
    }
}



#[derive(Debug)]
pub struct GodotPck {
    file: File,
    version: (i32, i32, i32, i32),
    flags: GodotPckFlags,
    num_files: i32,
    files: Vec<GodotPckFile>,
    encryption_key: Option<[u8; 32]>,
}

impl GodotPck {

    const IDENTIFIER: [u8; 4] = *b"GDPC";

    pub fn from_file(mut file: File, encryption_key: Option<[u8; 32]>) -> Result<Self, Box<dyn Error>> {
        assert!(Self::IDENTIFIER.iter().eq(file.read_primitive::<[u8; 4]>()?.iter()), "Archive identifier does not match.");

        let version: (i32, i32, i32, i32) = (
            file.read_primitive()?, file.read_primitive()?, file.read_primitive()?, file.read_primitive()?,
        );

        let flags = GodotPckFlags::from_bits_retain(if version.0 >= 2 { file.read_primitive()? } else { 0 });
        let files_offset: i64 = if version.0 >= 2 { file.read_primitive()? } else { 0 };
        file.seek(std::io::SeekFrom::Current(16 * 4))?;
        let num_files: i32 = file.read_primitive()?;

        let mut files = Vec::new();
        for _ in 0..num_files {
            let path_len: i32 = file.read_primitive()?;
            let mut path = String::from_utf8(file.read_to_vec(path_len as usize)?)?;
            // Path length is padded with '\0' to nearest 4 bytes.
            path = path.trim_matches('\0').to_string();

            let offset: i64 = file.read_primitive()?;
            let size: i64 = file.read_primitive()?;
            let md5: [u8; 16] = file.read_primitive()?;

            let flags = GodotPckFileFlags::from_bits_retain(if version.0 >= 2 { file.read_primitive()? } else { 0 });

            files.push(GodotPckFile { file: file.try_clone()?, path, offset: offset + files_offset, size, md5, flags, encryption_key });
        }

        Ok(GodotPck { file: file.try_clone()?, version, flags, num_files, files, encryption_key })
    }
    
}

impl VirtualDirectory<GodotPckFile, GodotPck> for GodotPck {
    fn path(&mut self) -> &str {
        ""
    }

    fn read_entries(&mut self) -> Result<Vec<VirtualEntry<GodotPckFile, GodotPck>>, Box<dyn Error>> {
        let mut entries: Vec<VirtualEntry<GodotPckFile, GodotPck>> = Vec::new();
        self.files.iter_mut().for_each(|file| {
            entries.push(VirtualEntry::File(file));
        });
        Ok(entries)
    }
}


