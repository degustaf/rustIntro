use std::env::args;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::Error as ioError;

fn load_source_code(fname: &str) -> Result<String, ioError> {
    let mut file = File::open(fname)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer
        .iter()
        .map(|u| char::from(*u))
        .filter(|c| "><+-.,[]".contains(*c))
        .collect::<String>())
}

fn main() -> Result<(), Box<dyn Error>> {
    let fname = args().nth(1).ok_or("A file name to process is required.")?;
    let src = load_source_code(&fname)?;

    println!("{}", src);

    Ok(())
}
