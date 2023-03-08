use std::collections::HashMap;
use std::convert::TryFrom;
use std::default::Default;
use std::env::args;
use std::error::Error;
use std::fmt;
use std::fmt::Display;
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
    type Error = Box<dyn Error>;

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
    type Error = Box<dyn Error>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Line::try_from(value.as_str())
    }
}

fn parse_file(fname: &str) -> std::result::Result<Vec<Line>, Box<dyn Error>> {
    BufReader::new(File::open(fname)?)
        .lines()
        .map(|l| Line::try_from(l?))
        .collect()
}

#[derive(Debug, Default)]
struct Scores {
    count: usize,
    total: f64,
    missed: usize,
}

impl Scores {
    fn add_score(&mut self, score: f64) {
        self.count += 1;
        self.total += score;
    }

    fn missed_test(&mut self) {
        self.missed += 1;
    }
}

fn tests(n: usize) -> &'static str {
    if n == 1 {
        "test"
    } else {
        "tests"
    }
}

impl Display for Scores {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}, with a total score of {}.  They missed {} {}",
            self.count,
            tests(self.count),
            self.total,
            self.missed,
            tests(self.missed)
        )
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let fname = args().nth(1).ok_or("A file name to process is required.")?;
    let data = parse_file(&fname)?;

    let mut map: HashMap<String, Scores> = HashMap::new();
    for l in data {
        match l {
            Line::Name(name) => {
                map.entry(name).or_default().missed_test();
            }
            Line::NameNumber { name, number } => {
                map.entry(name).or_default().add_score(number);
            }
        }
    }

    println!("{:?}", map);

    for (key, val) in &map {
        println!("{} took {}.", key, val);
    }

    Ok(())
}
