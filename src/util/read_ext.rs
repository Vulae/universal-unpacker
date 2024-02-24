
use std::{error::Error, io::{self, Read}};
use bitstream_io::{Numeric, Primitive};



pub trait ReadExt {

    fn read_primitive<V: Primitive>(&mut self) -> io::Result<V>;

    fn read_to_vec(&mut self, len: usize) -> io::Result<Vec<u8>>;

    fn read_string_len(&mut self, len: usize) -> Result<String, Box<dyn Error>> {
        Ok(String::from_utf8(self.read_to_vec(len as usize)?)?)
    }

    fn read_string<LV: Into<usize> + Primitive>(&mut self) -> Result<String, Box<dyn Error>> {
        let len = self.read_primitive::<LV>()?;
        self.read_string_len(len.into())
    }

    fn check_magic<V: Numeric + Into<usize> + Primitive>(&mut self, magic: V) -> Result<bool, Box<dyn Error>> {
        let v = self.read_primitive::<V>()?;
        Ok(
            v.to_le_bytes().as_ref()
            .eq(magic.to_le_bytes().as_ref())
        )
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


