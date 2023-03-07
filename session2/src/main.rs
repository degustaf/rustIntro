use std::convert::TryFrom;
// use std::convert::From;
// use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[allow(dead_code)]
#[derive(Debug)]
enum Line {
    Name(String),
    NameNumber { name: String, number: f64 },
}

impl TryFrom<&str> for Line {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.split_once(':') {
            None => Ok(Line::Name(value.to_string())),
            Some((name, number_string)) => Ok(Line::NameNumber {
                name: name.to_string(),
                number: f64::from_str(number_string)?,
            }),
        }
    }
}

impl TryFrom<String> for Line {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Line::try_from(value.as_str())
    }
}

#[allow(dead_code)]
fn parse_file(fname: &str) -> std::result::Result<Vec<Line>, Box<dyn std::error::Error>> {
    BufReader::new(File::open(fname)?)
        .lines()
        .map(|l| Line::try_from(l?))
        .collect()
}

fn main() {
    println!("Hello, world!");
}
