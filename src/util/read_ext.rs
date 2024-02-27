
use std::{error::Error, io::{self, Read}};
use bitstream_io::{Numeric, Primitive};



pub trait ReadExt: Read {

    fn read_primitive<V: Primitive>(&mut self) -> io::Result<V>;

    fn read_to_vec(&mut self, len: usize) -> io::Result<Vec<u8>>;

    fn read_string_len(&mut self, len: usize) -> Result<String, Box<dyn Error>> {
        Ok(String::from_utf8(self.read_to_vec(len as usize)?)?)
    }

    fn read_string<LV: TryInto<usize> + Primitive>(&mut self) -> Result<String, Box<dyn Error>> {
        let len = self.read_primitive::<LV>()?;
        self.read_string_len(len.try_into().map_err(|_| std::io::Error::new(io::ErrorKind::Other, "Failed to read string, try into usize failed."))?)
    }

    /// Read a string terminating in byte.
    /// 
    /// Useful to reading to null byte or newline.
    /// 
    /// String may become corrupt if terminator is not a control byte.
    fn read_terminated_string(&mut self, terminator: u8) -> Result<String, Box<dyn Error>> {
        let mut str: Vec<u8> = Vec::new();
        loop {
            let value: u8 = self.read_primitive()?;
            if value == terminator { break }
            str.push(value);
        }
        Ok(String::from_utf8(str)?)
    }

    fn check_magic<V: Numeric + Into<usize> + Primitive>(&mut self, magic: V) -> Result<bool, Box<dyn Error>> {
        let v = self.read_primitive::<V>()?;
        Ok(
            v.to_le_bytes().as_ref()
            .eq(magic.to_le_bytes().as_ref())
        )
    }

    fn check_magic_vec(&mut self, magic: Vec<u8>) -> Result<bool, Box<dyn Error>> {
        let bytes = self.read_to_vec(magic.len())?;
        if magic.len() != bytes.len() {
            return Ok(false);
        }
        for i in 0..magic.len() {
            if magic[i] != bytes[i] {
                return Ok(false);
            }
        }
        Ok(true)
    }

    fn check_magic_string<S: Into<String>>(&mut self, magic: S) -> Result<bool, Box<dyn Error>> {
        let magic: String = magic.into();
        self.check_magic_vec(magic.into_bytes())
    }

}



impl<T: Read> ReadExt for T {

    fn read_primitive<V: Primitive>(&mut self) -> io::Result<V> {
        let mut buffer = V::buffer();
        self.read_exact(buffer.as_mut())?;
        Ok(V::from_le_bytes(buffer))
    }

    fn read_to_vec(&mut self, len: usize) -> io::Result<Vec<u8>> {
        let mut buf = vec![0u8; len];
        self.read(&mut buf)?;
        Ok(buf)
    }

}


