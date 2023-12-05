use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

const PATH: &str = "input";
pub fn main() -> io::Result<()> {
    let f = File::open(PATH)?;
    let mut reader = BufReader::new(f);

    let mut buffer = String::new();
    let mut sum = 0;

    while reader.read_line(&mut buffer)? != 0 {
        sum += get_card_value(&buffer)?;
        buffer.clear();
    }

    println!("{sum}");

    Ok(())
}

fn get_card_value(entry: &str) -> io::Result<usize> {
    let winning_numbers = get_winning_numbers(entry).ok_or(io::Error::new(
        io::ErrorKind::InvalidInput,
        "Malformatted entry, no separator",
    ))?;
    let picked_numbers = get_picked_numbers(entry)?;
    let mut result = 0;
    for num in picked_numbers {
        if winning_numbers.contains(&num) {
            if result == 0 {
                result = 1;
            } else {
                result *= 2;
            }
        }
    }

    Ok(result)
}

fn get_winning_numbers(entry: &str) -> Option<Vec<usize>> {
    let mut iter = entry.split_ascii_whitespace().skip(2);
    let mut result = Vec::new();

    while let Ok(num) = usize::from_str_radix(iter.next()?, 10) {
        result.push(num);
    }
    Some(result)
}

fn get_picked_numbers(entry: &str) -> io::Result<Vec<usize>> {
    let mut iter = entry.split_ascii_whitespace().skip(2);
    let mut result = Vec::new();

    iter.find(|word| *word == "|").ok_or(io::Error::new(
        io::ErrorKind::InvalidInput,
        "Malformatted entry, no separator",
    ))?;
    for word in iter {
        result.push(
            usize::from_str_radix(word, 10)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?,
        );
    }

    Ok(result)
}
