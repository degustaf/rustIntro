use std::env::args;
use std::error::Error;
use std::fs::read_to_string;
// use std::io::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let fname = args().nth(1).ok_or("A file name to process is required.")?;
    let src: String = read_to_string(fname)?
        .chars()
        .filter(|c| "><+-.,[]".contains(*c))
        .collect();

    println!("{}", src);

    Ok(())
}
