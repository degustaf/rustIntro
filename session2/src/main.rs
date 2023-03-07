use std::convert::TryFrom;
use std::str::FromStr;

// Would a specific type for T be more a ppropriate?
#[allow(dead_code)]
#[derive(Debug)]
enum Line<T: FromStr> {
    Name(String),
    NameNumber { name: String, number: T },
}

impl<T: FromStr> TryFrom<&str> for Line<T> {
    type Error = <T as FromStr>::Err;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.split_once(':') {
            None => Ok(Line::Name(value.to_string())),
            Some((name, number_string)) => {
                let number = <T as FromStr>::from_str(number_string)?;
                Ok(Line::NameNumber {
                    name: name.to_string(),
                    number,
                })
            }
        }
    }
}

impl<T: FromStr> TryFrom<&String> for Line<T> {
    type Error = <T as FromStr>::Err;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Line::try_from(value.as_str())
    }
}

fn main() {
    println!("Hello, world!");
}
