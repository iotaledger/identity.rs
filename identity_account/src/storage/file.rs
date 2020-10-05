use std::{
    fs::OpenOptions,
    io::{BufReader, BufWriter, Read, Write},
};

pub struct CacheFile(String);

impl CacheFile {
    pub fn new(name: String) -> Self {
        CacheFile(name)
    }

    pub fn get_name(&self) -> &String {
        &self.0
    }

    pub fn write_cache_file(&self, input: Vec<u8>) -> crate::Result<()> {
        let file = OpenOptions::new().write(true).create(true).open(self.get_name())?;
        let mut buf_writer = BufWriter::new(file);
        buf_writer.write_all(&input)?;
        Ok(())
    }
    pub fn read_cache_file(&self) -> crate::Result<Vec<u8>> {
        let file = OpenOptions::new().read(true).open(self.get_name())?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = Vec::new();
        buf_reader.read_to_end(&mut contents)?;
        Ok(contents)
    }
}
