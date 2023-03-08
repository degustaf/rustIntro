use std::env::args;
use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum Instruction {
    MoveLeft,
    MoveRight,
    Increment,
    Decrement,
    Input,
    Output,
    BeginLoop,
    EndLoop,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::MoveLeft => write!(f, "Move left one location"),
            Self::MoveRight => write!(f, "Move right one location"),
            Self::Increment => write!(f, "Increment current location"),
            Self::Decrement => write!(f, "Decrement current location"),
            Self::Input => write!(f, "Accept one byte of input"),
            Self::Output => write!(f, "Output the current byte"),
            Self::BeginLoop => write!(f, "Start looping"),
            Self::EndLoop => write!(f, "Finish looping"),
        }
    }
}

impl Instruction {
    fn from_byte(c: u8) -> Option<Self> {
        match c {
            b'<' => Some(Instruction::MoveLeft),
            b'>' => Some(Instruction::MoveRight),
            b'+' => Some(Instruction::Increment),
            b'-' => Some(Instruction::Decrement),
            b',' => Some(Instruction::Input),
            b'.' => Some(Instruction::Output),
            b'[' => Some(Instruction::BeginLoop),
            b']' => Some(Instruction::EndLoop),
            _ => None,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct InputInstruction<'a> {
    inst: Instruction,
    line_number: usize,
    char_number: usize,
    source_name: &'a str,
}

impl<'a> Display for InputInstruction<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{}:{}:{}] {}",
            self.source_name, self.line_number, self.char_number, self.inst
        )
    }
}

fn load_source_code(source_name: &str) -> Result<Vec<InputInstruction>, Box<dyn Error>> {
    let file = BufReader::new(File::open(source_name)?);

    /*
     * I can't get this typecheck. Not sure what I'm doing wrong.
    Ok(file.split(b'\n').enumerate().flat_map(|(line_number, line)| Ok(line?
                .iter()
                .enumerate()
                .filter_map(|(char_number, c)| {
                    Instruction::from_byte(*c).and_then(|inst| Some((char_number, inst)))
                })
                .map(|(char_number, inst)| InputInstruction {
                    inst,
                    line_number: line_number + 1,
                    char_number: char_number + 1,
                    source_name,
                }))
        ).collect::())
    */

    /*
     * This works, but I think the for loop looks better
    let mut ret = Vec::new();
    for (line_number, line) in file.split(b'\n').enumerate() {
        ret.extend(
            line?
                .iter()
                .enumerate()
                .filter_map(|(char_number, c)| {
                    Instruction::from_byte(*c).and_then(|inst| Some((char_number, inst)))
                })
                .map(|(char_number, inst)| InputInstruction {
                    inst,
                    line_number: line_number + 1,
                    char_number: char_number + 1,
                    source_name,
                }),
        );
    }
    Ok(ret)
    */

    let mut ret = Vec::new();
    // Technically we should split on b'\n', b'\r\n', or '\r'.
    // b'\r\n' will leave a b'\r' at the end of the line, this will be consumed without issue.
    // b'\r' was only used as a line terminator by Macs, pre OS X. We'll assume that this won't
    // be an issue...
    for (line_number, line) in file.split(b'\n').enumerate() {
        for (char_number, c) in line?.iter().enumerate() {
            if let Some(inst) = Instruction::from_byte(*c) {
                ret.push(InputInstruction {
                    inst,
                    line_number: line_number + 1,
                    char_number: char_number + 1,
                    source_name,
                });
            }
        }
    }
    Ok(ret)
}

fn main() -> Result<(), Box<dyn Error>> {
    let fname = args().nth(1).ok_or("A file name to process is required.")?;
    let src = load_source_code(&fname)?;

    for inst in src {
        println!("{}", inst);
    }

    Ok(())
}
