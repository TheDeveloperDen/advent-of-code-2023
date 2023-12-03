use std::fs::File;
use std::io::{self, BufRead, BufReader};

const PATH: &str = "input";
const SEPARATORS: [char; 2] = [',', ';'];
pub fn main() -> io::Result<()> {
    let f = File::open(PATH)?;
    let mut reader = BufReader::new(f);

    let mut buffer = String::new();
    let mut sum = 0;

    while reader.read_line(&mut buffer)? != 0 {
        let result = is_within_limits(&buffer)?;
        if result {
            sum += get_game_id(&buffer)?;
        }

        buffer.clear();
    }

    println!("{}", sum);
    Ok(())
}

fn get_game_id(entry: &str) -> io::Result<usize> {
    let start_idx = 5;
    let end_idx = entry.find(':').ok_or(io::Error::new(
        io::ErrorKind::InvalidInput,
        format!("Malformatted entry: {entry}"),
    ))?;

    let id_str = &entry[start_idx..end_idx];
    usize::from_str_radix(id_str, 10).map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))
}

fn is_within_limits(entry: &str) -> io::Result<bool> {
    let iter = entry.split_whitespace().skip(2);
    let mut number_next = true;
    let mut last_number = 0;
    for mut elem in iter {
        if number_next {
            last_number = usize::from_str_radix(elem, 10)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
        } else {
            elem = elem.strip_suffix(&SEPARATORS).unwrap_or(elem);
            let limit = get_limit_for(elem).ok_or(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Unknown color: {elem}"),
            ))?;
            if last_number > limit {
                return Ok(false);
            }
        }
        number_next = !number_next;
    }

    Ok(true)
}

fn get_limit_for(color: &str) -> Option<usize> {
    match color {
        "red" => Some(12),
        "green" => Some(13),
        "blue" => Some(14),
        _ => None,
    }
}
