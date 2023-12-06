use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    str::SplitAsciiWhitespace,
};

struct DataPoint {
    value: usize,
    updated: bool,
}

impl DataPoint {
    fn update_if_needed(
        &mut self,
        dest_range_start: usize,
        source_range_start: usize,
        range_length: usize,
    ) {
        if self.should_be_updated(source_range_start, range_length) {
            self.value = self.value - source_range_start + dest_range_start;
            self.updated = true;
        }
    }

    fn should_be_updated(&self, source_range_start: usize, range_length: usize) -> bool {
        self.is_in_range(source_range_start, range_length) && !self.updated
    }

    fn is_in_range(&self, source_range_start: usize, range_length: usize) -> bool {
        self.value >= source_range_start && self.value < source_range_start + range_length
    }
}

const PATH: &str = "input";
pub fn main() -> io::Result<()> {
    let f = File::open(PATH)?;
    let mut reader = BufReader::new(f);

    let mut buffer = String::new();

    if reader.read_line(&mut buffer)? == 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Empty input file",
        ));
    }

    let mut data = get_seeds(&buffer)?;
    buffer.clear();

    while reader.read_line(&mut buffer)? != 0 {
        if !buffer.starts_with(|ch: char| ch.is_ascii_digit()) {
            if buffer.starts_with(|ch: char| ch.is_ascii_whitespace()) {
                for val in &mut data {
                    val.updated = false;
                }
            }
        } else {
            let mut iter = buffer.split_ascii_whitespace();
            let dest_range_start = get_next_number_after_split(&mut iter)?;
            let source_range_start = get_next_number_after_split(&mut iter)?;
            let range_length = get_next_number_after_split(&mut iter)?;

            for data_point in &mut data {
                data_point.update_if_needed(dest_range_start, source_range_start, range_length);
            }
        }
        buffer.clear();
    }

    println!(
        "{}",
        get_smallest_data_point(&data).ok_or(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Empty data Vec"
        ))?
    );
    Ok(())
}

fn get_seeds(entry: &str) -> io::Result<Vec<DataPoint>> {
    let mut data = Vec::new();
    for val in entry.split_ascii_whitespace().skip(1) {
        data.push(DataPoint {
            value: usize::from_str_radix(val, 10)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?,
            updated: false,
        });
    }

    Ok(data)
}

fn get_next_number_after_split(iter: &mut SplitAsciiWhitespace<'_>) -> io::Result<usize> {
    Ok(usize::from_str_radix(
        iter.next().ok_or(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Tried to read from a finished iterator",
        ))?,
        10,
    )
    .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?)
}

fn get_smallest_data_point(data: &Vec<DataPoint>) -> Option<usize> {
    Some(data.iter().min_by(|x, y| x.value.cmp(&y.value))?.value)
}
