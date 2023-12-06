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
        sum += get_power_of_set(&buffer)?;

        buffer.clear();
    }

    println!("{}", sum);
    Ok(())
}

#[derive(Default)]
struct Colors {
    red: usize,
    green: usize,
    blue: usize,
}

impl Colors {
    fn map_color_to_field(self: &mut Self, color: &str) -> Option<&mut usize> {
        match color {
            "red" => Some(&mut self.red),
            "green" => Some(&mut self.green),
            "blue" => Some(&mut self.blue),
            _ => None,
        }
    }
}

//set (noun)
fn get_power_of_set(entry: &str) -> io::Result<usize> {
    let mut colors = Colors::default();
    let iter = entry.split_whitespace().skip(2);

    let mut number_next = true;
    let mut last_number = 0;

    for mut elem in iter {
        if number_next {
            last_number = usize::from_str_radix(elem, 10)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
        } else {
            elem = elem.strip_suffix(&SEPARATORS).unwrap_or(elem);

            let color_count = colors.map_color_to_field(elem).ok_or(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Malformatted color name: {elem}"),
            ))?;
            if last_number > *color_count {
                *color_count = last_number;
            }
        }

        number_next = !number_next;
    }

    Ok(colors.red * colors.green * colors.blue)
}
