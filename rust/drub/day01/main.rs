//thanks to Lena for help
use std::fs::File;
use std::io::{self, BufRead, BufReader};

const PATH: &str = "input";
const DIGIT_NAMES: &[(&str, u32)] = &[
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
    ("0", 0),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
];

fn main() -> io::Result<()> {
    let f = File::open(PATH)?;
    let mut reader = BufReader::new(f);

    let mut buffer = String::new();
    let mut sum = 0;
    while reader.read_line(&mut buffer)? != 0 {
        sum += return_first_digit(&buffer)? * 10;
        sum += return_last_digit(&buffer)?;
        /*
        //alternative, for the first part of the challenge only
        sum += return_first_digit(buffer.chars())? * 10;
        sum += return_first_digit(buffer.chars().rev())?;
        */
        buffer.clear();
    }

    println!("{}", sum);
    Ok(())
}

macro_rules! make_return_some_digit {
    ($fun_name:ident, $check_name:ident, $strip_name:ident) => {
        fn $fun_name(mut line: &str) -> io::Result<u32> {
            while !line.is_empty() {
                for (name, value) in DIGIT_NAMES {
                    if line.$check_name(*name) {
                        return Ok(*value);
                    }
                }
                line = line.$strip_name(|_| true).unwrap_or_default();
            }
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "No calibration value",
            ))
        }
    };
}

make_return_some_digit!(return_first_digit, starts_with, strip_prefix);
make_return_some_digit!(return_last_digit, ends_with, strip_suffix);

/*
//alternative code, supports only the first part of the challenge
fn return_first_digit<I>(iter: I) -> io::Result<u32>
where
    I: Iterator<Item = char>,
{
    for ch in iter {
        if let Some(value) = ch.to_digit(10) {
            return Ok(value);
        }
    }
    Err(io::Error::new(
        io::ErrorKind::InvalidInput,
        "No calibration value",
    ))
}
*/
