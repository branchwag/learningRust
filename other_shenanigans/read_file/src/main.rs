use std::fs::{File, read};
use std::io::{self, Read};

fn read_file() -> io::Result<String> {
    let mut file = File::open("hello.txt")?;
    //if this is an error, return it immediately, otherwise unwrap success value
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() -> io::Result<()> {
    let contents = read_file()?;
    println!("{contents}");
    Ok(())
}
