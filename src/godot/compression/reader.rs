
use std::{error::Error, io::{self, Read, Seek, SeekFrom}};
use bitstream_io::Primitive;
use super::compression::Compression;





trait ReadExt {
    fn read_numeric<V: Primitive>(&mut self) -> io::Result<V>;
}

impl<T: Read> ReadExt for T {
    fn read_numeric<V: Primitive>(&mut self) -> io::Result<V> {
        let mut buffer = V::buffer();
        self.read_exact(buffer.as_mut())?;
        Ok(V::from_le_bytes(buffer))
    }
}





#[derive(Debug)]
struct Block {
    offset: u64,
    size: u32,
}



#[derive(Debug)]
pub struct GodotCompressedReader<'a, R: Read + Seek> {
    data: &'a mut R,
    compression: Compression,
    block_size: u32,
    read_total: u32,
    blocks: Vec<Block>,
    cur_block_index: usize,
    cur_block_pointer: usize,
    cur_block: Vec<u8>,
}

impl <'a, R: Read + Seek>GodotCompressedReader<'a, R> {

    pub fn open_after_ident(data: &'a mut R) -> Result<Self, Box<dyn Error>> {
        let compression = Compression::from(data.read_numeric()?)?;
        let block_size: u32 = data.read_numeric()?;
        let read_total: u32 = data.read_numeric()?;
        let num_blocks: u32 = (read_total / block_size) + 1;
        let mut blocks: Vec<Block> = Vec::new();
        
        let mut block_offset: u64 = data.stream_position()? + (num_blocks as u64) * 4;
        for _ in 0..num_blocks {
            let size: u32 = data.read_numeric()?;
            block_offset += size as u64;
            blocks.push(Block { offset: block_offset, size });
        }

        let mut compressed_reader = Self {
            data,
            compression,
            block_size,
            read_total,
            blocks,
            cur_block_index: 0,
            cur_block_pointer: 0,
            cur_block: Vec::new(),
        };
        compressed_reader.cur_block_pointer = 0;
        compressed_reader.get_block(compressed_reader.cur_block_index)?;

        Ok(compressed_reader)
    }

    const IDENTIFIER: [u8; 4] = *b"GCMP";

    pub fn open(data: &'a mut R) -> Result<Self, Box<dyn Error>> {
        assert!(Self::IDENTIFIER.iter().eq(data.read_numeric::<[u8; 4]>()?.iter()), "Compressed identifier does not match.");

        Self::open_after_ident(data)
    }



    pub fn length(&mut self) -> usize {
        self.read_total as usize
    }



    fn get_block(&mut self, block: usize) -> Result<Vec<u8>, Box<dyn Error>> {
        let block = &self.blocks[block];

        let mut block_data: Vec<u8> = Vec::with_capacity(block.size as usize);
        self.data.seek(std::io::SeekFrom::Start(block.offset))?;
        self.data.read(&mut block_data)?;

        let decompressed_block_data = self.compression.decompress(block_data)?;

        Ok(decompressed_block_data)
    }



    fn internal_read_u8(&mut self) -> Result<u8, Box<dyn Error>> {
        let value = self.cur_block[self.cur_block_pointer];

        if self.cur_block_pointer >= self.cur_block.len() {
            self.cur_block_index += 1;
            self.cur_block_pointer = 0;
            self.cur_block = self.get_block(self.cur_block_index)?;
        }

        Ok(value)
    }

    fn internal_read(&mut self, buf: &mut [u8]) -> Result<usize, Box<dyn Error>> {
        let mut buf_pointer: usize = 0;

        while buf_pointer < buf.len() {
            // TODO: Don't read like this.
            // Instead read the max amount the block buffer can then do the increment math.
            buf[buf_pointer] = self.internal_read_u8()?;
            buf_pointer += 1;
        }

        Ok(buf_pointer)
    }



    fn position(&mut self) -> usize {
        self.cur_block_index * (self.block_size as usize) + self.cur_block_pointer
    }

    fn internal_seek(&mut self, pos: std::io::SeekFrom) -> Result<u64, Box<dyn Error>> {
        match pos {
            SeekFrom::Start(offset) => {
                let block_index = (offset / (self.block_size as u64)) as usize;

                if block_index != self.cur_block_index {
                    self.cur_block_index = block_index;
                    self.cur_block = self.get_block(block_index)?;
                }

                self.cur_block_pointer = (offset % (self.block_size as u64)) as usize;
            },
            SeekFrom::Current(offset) => {
                let cur_pos = self.position();
                self.internal_seek(SeekFrom::Start(((cur_pos as i64) + offset) as u64))?;
            },
            SeekFrom::End(offset) => {
                self.internal_seek(SeekFrom::Start(((self.read_total as i64) + offset) as u64))?;
            },
        }
        Ok(self.position() as u64)
    }

}

impl<'a, R: Read + Seek> Read for GodotCompressedReader<'a, R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        self.internal_read(buf).map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Error."))
    }
}

impl<'a, R: Read + Seek> Seek for GodotCompressedReader<'a, R> {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        self.internal_seek(pos).map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Error."))
    }
}


