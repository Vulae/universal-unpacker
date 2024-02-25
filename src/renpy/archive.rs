
use std::{error::Error, fmt, fs::File, io::{Cursor, Read, Seek, Write}};
use crate::util::{decode_hex, pickle::{parser::PickleParser, pickle::Pickle}, read_ext::ReadExt};



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
    pub path: String,
    pub size: u64,
    pub chunks: Vec<(u64, u64)>,
}

impl RenPyArchiveFile {
    
    pub fn read_data(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut buf = vec![0u8; self.size as usize];
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
    pub files: Vec<RenPyArchiveFile>,
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

        // TODO: Clean this shit up!
        match pickle {
            Pickle::Dict(dict) => {
                for (filename, chunks) in dict {
                    match chunks {
                        Pickle::List(list) => {
                            let mut file_chunks = Vec::new();
                            for chunk in list {
                                match chunk {
                                    Pickle::Tuple3((offset, length, _)) => {
                                        let offset: u64 = (*offset).try_into()?;
                                        let length: u64 = (*length).try_into()?;
                                        file_chunks.push((offset ^ xor, length ^ xor));
                                    },
                                    _ => return Err(Box::new(RenPyError::PickleParseFail)),
                                }
                            }
                            files.push(RenPyArchiveFile {
                                file: file.try_clone()?,
                                path: filename,
                                size: file_chunks.iter().fold(0, |size, chunk| size + chunk.1),
                                chunks: file_chunks
                            });
                        }
                        _ => return Err(Box::new(RenPyError::PickleParseFail)),
                    }
                }
            },
            _ => return Err(Box::new(RenPyError::PickleParseFail)),
        }

        Ok(RenPyArchive { files })
    }

}


