use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

struct Range {
    lower: usize,
    upper: usize,
}

impl Range {
    fn get_length(&self) -> usize {
        self.upper - self.lower + 1
    }
}

struct RaceData {
    time: usize,
    distance: Option<usize>,
}

impl RaceData {
    fn get_range_to_beat(&self) -> Option<Range> {
        let delta_sqrt: f64 = ((self.time * self.time - (4 * self.distance?)) as f64).sqrt();
        let mut lower = (self.time as f64 - delta_sqrt) / 2f64;
        let mut upper = (self.time as f64 + delta_sqrt) / 2f64;
        if (lower.ceil() as usize) * (self.time - lower.ceil() as usize) == self.distance? {
            lower += 1f64;
        }
        if (upper.floor() as usize) * (self.time - upper.floor() as usize) == self.distance? {
            upper -= 1f64;
        }
        Some(Range {
            lower: (lower).ceil() as usize,
            upper: (upper).floor() as usize,
        })
    }
}

const PATH: &str = "input";
pub fn main() -> io::Result<()> {
    let f = File::open(PATH)?;
    let mut reader = BufReader::new(f);

    let mut buffer = String::new();

    let mut race_data = Vec::new();

    while reader.read_line(&mut buffer)? != 0 {
        let mut iter = buffer.split_ascii_whitespace();
        if let Some(word) = iter.next() {
            match word {
                "Time:" => {
                    for num in iter {
                        race_data.push(RaceData {
                            time: usize::from_str_radix(num, 10)
                                .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?,
                            distance: None,
                        });
                    }
                }
                "Distance:" => {
                    for (idx, num) in iter.enumerate() {
                        if idx >= race_data.len() {
                            return Err(io::Error::new(
                                io::ErrorKind::InvalidInput,
                                "Too many distances in race data",
                            ));
                        }
                        race_data[idx].distance = Some(
                            usize::from_str_radix(num, 10)
                                .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?,
                        );
                    }
                }
                _ => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("Invalid input: {word}"),
                    ))
                }
            };
        }
        buffer.clear();
    }

    let mut result = 1;

    for data in race_data {
        result *= data
            .get_range_to_beat()
            .ok_or(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Too many times in race data",
            ))?
            .get_length();
    }

    println!("{result}");

    Ok(())
}
