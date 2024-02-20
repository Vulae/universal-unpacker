
use std::{error::Error, fs::File, io::{Read, Seek}};
use bitstream_io::{ByteRead, ByteReader, LittleEndian};
use bitflags::bitflags;



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
    pub path: String,
    pub offset: i64,
    pub size: i64,
    pub md5: [u8; 16],
    pub flags: GodotPckFileFlags,
}

impl GodotPckFile {

    pub fn read_data(&mut self, encryption_key: Option<[u8; 32]>) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut buf: Vec<u8> = vec![0; self.size as usize];
        self.file.seek(std::io::SeekFrom::Start(self.offset as u64))?;
        self.file.read(&mut buf)?;

        if self.flags.contains(GodotPckFileFlags::ENCRYPTED_FILE) {
            if let Some(_encryption_key) = encryption_key {
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
    pub file: File,
    pub version: (i32, i32, i32, i32),
    pub flags: GodotPckFlags,
    pub num_files: i32,
    pub files: Vec<GodotPckFile>,
}

impl GodotPck {

    pub fn from_file(file: File) -> Result<Self, Box<dyn Error>> {
        let mut reader = ByteReader::endian(&file, LittleEndian);

        assert!(reader.read::<u32>()? == 0x43504447, "GodotPck magic check failed.");

        let version: (i32, i32, i32, i32) = (
            reader.read()?, reader.read()?, reader.read()?, reader.read()?,
        );

        let flags = GodotPckFlags::from_bits_retain(if version.0 >= 2 { reader.read()? } else { 0 });
        let files_offset: i64 = if version.0 >= 2 { reader.read()? } else { 0 };
        reader.skip(16 * 4)?; // Reserved space
        let num_files: i32 = reader.read()?;

        let mut files = Vec::new();
        for _ in 0..num_files {
            let path_len: i32 = reader.read()?;
            let mut path = String::from_utf8(reader.read_to_vec(path_len as usize)?)?;
            // Path length is padded with '\0' to nearest 4 bytes.
            path = path.trim_matches('\0').to_string();

            let offset: i64 = reader.read()?;
            let size: i64 = reader.read()?;
            let md5: [u8; 16] = reader.read()?;

            let flags = GodotPckFileFlags::from_bits_retain(if version.0 >= 2 { reader.read()? } else { 0 });

            files.push(GodotPckFile { file: file.try_clone()?, path, offset: offset + files_offset, size, md5, flags });
        }

        Ok(GodotPck { file: file.try_clone()?, version, flags, num_files, files })
    }
    
}


