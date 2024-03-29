
use std::{collections::HashMap, error::Error, fmt, fs::File, io::{Cursor, Read, Seek, Write}};
use crate::util::{decode_hex, pickle::{parser::PickleParser, pickle::Pickle}, read_ext::ReadExt, virtual_fs::{VirtualDirectory, VirtualEntry, VirtualFile}};



#[derive(Debug)]
enum RenPyError {
    ArchiveInvalidHeader,
    PickleParseFail,
}


impl fmt::Display for RenPyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ArchiveInvalidHeader => write!(f, "Invalid Ren\'Py archive header"),
            Self::PickleParseFail => write!(f, "Pickle parse fail."),
        }
    }
}

impl Error for RenPyError { }





#[derive(Debug)]
pub struct RenPyArchiveFile {
    file: File,
    path: String,
    chunks: Vec<(u64, u64)>,
}

impl VirtualFile for RenPyArchiveFile {
    fn path(&mut self) -> &str {
        &self.path
    }

    fn read_data(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        let size = self.chunks.iter().fold(0, |total, (_, size)| total + size);

        let mut buf = vec![0u8; size as usize];
        let mut buf_cursor = Cursor::new(&mut buf);

        for chunk in &self.chunks {
            let mut chunk_buf = vec![0u8; chunk.1 as usize];
            self.file.seek(std::io::SeekFrom::Start(chunk.0 as u64))?;
            self.file.read(&mut chunk_buf)?;
            buf_cursor.write(&chunk_buf)?;
        }

        Ok(buf)
    }
}



#[derive(Debug)]
pub struct RenPyArchive {
    files: Vec<RenPyArchiveFile>,
}

impl RenPyArchive {

    pub fn from_file(mut file: File) -> Result<Self, Box<dyn Error>> {

        // RPA-X.X XXXXXXXXXXXXXXXX XXXXXXXX\n
        let header = file.read_string_len(34)?;

        if !header.ends_with("\n") {
            return Err(Box::new(RenPyError::ArchiveInvalidHeader));
        }

        let (str_ver, str_ofs, str_xor) = {
            let split = header.trim().split(" ").collect::<Vec<&str>>();
            if split.len() != 3 {
                return Err(Box::new(RenPyError::ArchiveInvalidHeader));
            }
            (split[0], split[1], split[2])
        };

        if str_ver != "RPA-3.0" {
            return Err(Box::new(RenPyError::ArchiveInvalidHeader));
        }

        let offset: u64 = Cursor::new({
            let mut buf = decode_hex(str_ofs)?;
            buf.reverse();
            buf
        }).read_primitive()?;
        let xor: u64 = Cursor::new({
            let mut buf = decode_hex(str_xor)?;
            buf.reverse();
            buf
        }).read_primitive::<u32>()? as u64;
        


        file.seek(std::io::SeekFrom::Start(offset))?;

        let mut encoded = Vec::new();
        file.read_to_end(&mut encoded)?;
        let mut decoded = Vec::new();
        let mut decoder = flate2::read::ZlibDecoder::new(Cursor::new(encoded));
        decoder.read_to_end(&mut decoded)?;

        let pickle = PickleParser::parse(&mut Cursor::new(decoded))?;
        
        // println!("Pickle: {:#?}", pickle);
        
        let mut files = Vec::new();
        
        for (path, pickle_chunks) in TryInto::<HashMap<String, Pickle>>::try_into(pickle.clone())? {
            let mut chunks = Vec::new();
            for chunk in TryInto::<Vec<Pickle>>::try_into(pickle_chunks)? {
                let chunk = TryInto::<(Pickle, Pickle, Pickle)>::try_into(chunk)?;
                let offset = TryInto::<u64>::try_into(chunk.0)?;
                let length = TryInto::<u64>::try_into(chunk.1)?;
                chunks.push((offset ^ xor, length ^ xor));
            }
            
            files.push(RenPyArchiveFile { file: file.try_clone()?, path, chunks });
        }

        Ok(RenPyArchive { files })
    }

}

impl VirtualDirectory<RenPyArchiveFile, RenPyArchive> for RenPyArchive {
    fn path(&mut self) -> &str {
        ""
    }

    fn read_entries(&mut self) -> Result<Vec<VirtualEntry<RenPyArchiveFile, RenPyArchive>>, Box<dyn Error>> {
        let mut entries: Vec<VirtualEntry<RenPyArchiveFile, RenPyArchive>> = Vec::new();
        self.files.iter_mut().for_each(|file| {
            entries.push(VirtualEntry::File(file));
        });
        Ok(entries)
    }
}
