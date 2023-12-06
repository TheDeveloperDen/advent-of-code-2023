use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

const PATH: &str = "input";
pub fn main() -> io::Result<()> {
    let f = File::open(PATH)?;
    let mut reader = BufReader::new(f);

    let mut buffer = String::new();
    let mut card_count = Vec::new();

    let mut current_card_idx = 0;
    while reader.read_line(&mut buffer)? != 0 {
        let current_card_value = get_card_value(&buffer)?;
        let current_card_amount = match card_count.get(current_card_idx) {
            None => {
                card_count.push(1);
                1
            }
            Some(num) => *num,
        };
        for won_card_idx in (current_card_idx + 1)..=(current_card_idx + current_card_value) {
            match card_count.get(won_card_idx) {
                None => card_count.push(current_card_amount + 1),
                Some(won_card_amount) => {
                    card_count[won_card_idx] = current_card_amount + won_card_amount
                }
            };
        }
        current_card_idx += 1;
        buffer.clear();
    }

    println!("{}", card_count.iter().sum::<usize>());
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
            result += 1;
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
