use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Lines, Write};
use std::io::prelude::*;

fn main() {
    // Create a file and fill it with data
    let path = "./foo.txt";
    println!("Writing some data to '{}'", path);
    write_file(path, "Hello World!\n").expect("Failed to write to file");
    // Read entire file as a string
    let content = read_file(path).expect("Failed to read file");
    println!("The file '{}' contains:", path);
    println!("{}", content);

    // Overwrite the file
    println!("Writing new data to '{}'", path);
    write_file(path, "New content\n").expect("Failed to write to file");
    let content = read_file(path).expect("Failed to read file");
    println!("The file '{}' now contains:", path);
    println!("{}", content);

    // Append data to the file
    println!("Appending data to '{}'", path);
    append_file(path, "Some more content\n").expect("Failed to append to file");
    println!("The file '{}' now contains:", path);
    // Read file line by line as an iterator
    let lines = read_file_iterator(path).expect("Failed to read file");
    for line in lines {
        println!("{}", line.expect("Failed to read line"));
    }
}



fn read_file(path: &str) -> io::Result<String> {
    // open() opens the file in read-only mode
    let file = File::open(path)?;
    // Wrap the file in a BufReader
    // to read in an efficient way
    let mut buf_reader = BufReader::new(file);
    let mut content = String::new();
    buf_reader.read_to_string(&mut content)?;
    Ok(content)
}

fn read_file_iterator(path: &str) -> io::Result<Lines<BufReader<File>>> {
    let file = File::open(path)?;
    let buf_reader = BufReader::new(file);
    // lines() returns an iterator over lines
    Ok(buf_reader.lines())
}


fn write_file(path: &str, content: &str) -> io::Result<()> {
    // create() opens a file with the standard options
    // to create, write and truncate a file
    let file = File::create(path)?;
    // Wrap the file in a BufReader
    // to read in an efficient way
    let mut buf_writer = BufWriter::new(file);
    buf_writer.write_all(content.as_bytes())?;
    Ok(())
}

fn append_file(path: &str, content: &str) -> io::Result<()> {
    // OpenOptions let's you set all options individually
    let file = OpenOptions::new().append(true).open(path)?;
    let mut buf_writer = BufWriter::new(file);
    buf_writer.write_all(content.as_bytes())?;
    Ok(())
}
