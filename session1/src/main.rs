#[macro_use]
extern crate derive_error;

use std::env::args;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug, Error)]
enum ParseNumError {
    ParseIntError(std::num::ParseIntError),
    ParseFloatError(std::num::ParseFloatError),
}

fn parse(val: &str) -> Result<f64, ParseNumError> {
    val.parse::<f64>().or_else(|e| {
        val.strip_prefix("0x").map_or_else(
            || Err(ParseNumError::ParseFloatError(e)),
            |s| Ok(i64::from_str_radix(s, 16)? as f64),
        )
    })
}

fn main() -> Result<(), Box<dyn Error>> {
    // processing multiple files might be desirable, but is a change infunctionality.
    let fname = args().nth(1).ok_or("A file name to process is required.")?;
    let file = File::open(fname)?;
    let buf = BufReader::new(file);

    let mut acc = 0.0;
    for line in buf.lines() {
        acc += parse(&line?)?;
    }
    println!("The total is {}", acc);

    Ok(())
}
