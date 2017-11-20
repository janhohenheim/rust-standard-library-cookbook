extern crate byteorder;
use std::io::{Cursor, Seek, SeekFrom};
use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Lines, SeekFrom, Write};
use std::io::prelude::*;


fn main() {
    // Create a file and fill it with data
    let path = "./bar.bin";
    append_and_read(path, b"Last line in the file, goodbye").expect("Failed to read and write file");
}



fn append_and_read(path: &str, content: &[u8]) -> io::Result<()> {
    let file = OpenOptions::new().read(true).append(true).open(path)?;
    // Passing a reference of the file will not move it
    // allowing you to create both a reader and a writer
    let mut buf_reader = BufReader::new(&file);
    let mut buf_writer = BufWriter::new(&file);

    let mut file_content = String::new();
    buf_reader.read_to_string(&mut file_content)?;
    println!("File before appending:\n{}", file_content);

    // Appending will shift your positional pointer
    // so you have to save and restore it
    let pos = buf_reader.seek(SeekFrom::Current(0))?;
    buf_writer.write_all(content.as_bytes())?;
    // Flushing forces the write to happen right now
    buf_writer.flush()?;
    buf_reader.seek(SeekFrom::Start(pos))?;

    buf_reader.read_to_string(&mut file_content)?;
    println!("File after appending:\n{}", file_content);

    Ok(())
}
