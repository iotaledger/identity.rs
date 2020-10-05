use std::{
    fs::OpenOptions,
    io::{BufReader, BufWriter, Read, Write},
};

/// A newtype abstraction for a cached file.  Contains a `String` as the file's path.
pub struct CacheFile(String);

impl CacheFile {
    /// Create a new `CacheFile`
    pub fn new(name: String) -> Self {
        CacheFile(name)
    }

    /// get the filename from the `CacheFile`
    pub fn get_name(&self) -> &String {
        &self.0
    }

    /// write the input; `Vec<u8>` to the cache file.
    pub fn write_cache_file(&self, input: Vec<u8>) -> crate::Result<()> {
        let file = OpenOptions::new().write(true).create(true).open(self.get_name())?;
        let mut buf_writer = BufWriter::new(file);
        buf_writer.write_all(&input)?;
        Ok(())
    }

    /// read the contents from the cache file as a `Vec<u8>`
    pub fn read_cache_file(&self) -> crate::Result<Vec<u8>> {
        let file = OpenOptions::new().read(true).open(self.get_name())?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = Vec::new();
        buf_reader.read_to_end(&mut contents)?;
        Ok(contents)
    }
}
