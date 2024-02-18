
use std::{error::Error, fs::{self, File}, io::{Read, Seek, Write}, os::windows::fs::FileExt, path::PathBuf};
use bitstream_io::{ByteRead, ByteReader, LittleEndian};



#[derive(Debug)]
struct GodotPckFile {
    pub path: String,
    pub offset: i64,
    pub size: i64,
    pub md5: [u8; 16],
}

#[derive(Debug)]
struct GodotPck {
    pub file: File,
    pub version: (i32, i32, i32, i32),
    pub num_files: i32,
    pub files: Vec<GodotPckFile>,
}

impl GodotPck {
    fn from_file(file: File) -> Result<Self, Box<dyn Error>> {
        let mut reader = ByteReader::endian(&file, LittleEndian);

        assert!(reader.read::<u32>()? == 0x43504447, "GodotPck magic check failed.");

        let version: (i32, i32, i32, i32) = (
            reader.read()?, reader.read()?, reader.read()?, reader.read()?,
        );

        if version.0 >= 2 {
            let flags: u32 = reader.read()?;
            assert!((flags & 1) == 0, "GodotPck cannot extract encrypted archive.");
        }

        let files_offset: i64 = if version.0 >= 2 { reader.read()? } else { 0 };
        reader.skip(16 * 4)?; // Reserved space
        let num_files: i32 = reader.read()?;

        let mut files: Vec<GodotPckFile> = Vec::new();
        for _ in 0..num_files {
            let path_len: i32 = reader.read()?;
            let mut path = String::from_utf8(reader.read_to_vec(path_len as usize)?)?;
            // Path length is padded with '\0' to nearest 4 bytes.
            path = path.trim_matches('\0').to_string();

            let offset: i64 = reader.read()?;
            let size: i64 = reader.read()?;
            let md5: [u8; 16] = reader.read()?;

            if version.0 >= 2 {
                let flags: u32 = reader.read()?;
                assert!((flags & 1) == 0, "GodotPck cannot extract encrypted file.");
            }

            files.push(GodotPckFile { path, offset: offset + files_offset, size, md5 });
        }

        Ok(GodotPck { file, version, num_files, files })
    }
}



pub fn extract(output_path: &PathBuf, file_path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let file = fs::File::open(file_path)?;

    let mut godot_pck = GodotPck::from_file(file)?;
    
    for file in &godot_pck.files {
        let mut buf: Vec<u8> = vec![0; file.size as usize];
        godot_pck.file.seek(std::io::SeekFrom::Start(file.offset as u64))?;
        godot_pck.file.read(&mut buf)?;
        
        let mut output_file_path = PathBuf::from(output_path);
        output_file_path.push(&file.path.trim_start_matches("res://"));

        fs::create_dir_all(output_file_path.parent().unwrap())?;

        let mut output_file = File::create(output_file_path)?;
        output_file.write_all(&mut buf)?;
        output_file.flush()?;
    }

    println!("{:#?}", &godot_pck);

    Ok(())
}


