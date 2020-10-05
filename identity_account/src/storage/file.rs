use std::{
    fs::OpenOptions,
    io::{BufReader, BufWriter, Read, Write},
};

pub fn write_cache_file(input: Vec<u8>, filename: &String) -> crate::Result<()> {
    let file = OpenOptions::new().write(true).create(true).open(filename)?;

    let mut buf_writer = BufWriter::new(file);

    buf_writer.write(&input)?;

    Ok(())
}

pub fn read_cache_file(filename: &String) -> crate::Result<Vec<u8>> {
    let file = OpenOptions::new().read(true).open(filename)?;

    let mut buf_reader = BufReader::new(file);
    let mut contents = Vec::new();

    buf_reader.read_to_end(&mut contents)?;

    Ok(contents)
}
