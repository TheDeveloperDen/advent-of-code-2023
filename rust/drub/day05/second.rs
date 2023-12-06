use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    str::SplitAsciiWhitespace,
};

#[derive(PartialEq, Eq, Debug)]
struct SeedRange {
    start: usize,
    end: usize,
    updated: bool,
}

impl Ord for SeedRange {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start.cmp(&other.start)
    }
}

impl PartialOrd for SeedRange {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl SeedRange {
    fn overlaps_with(&self, other: &Self) -> bool {
        self.start <= other.end && self.end >= other.start
    }

    fn join_if_overlapping(&self, other: &Self, updated: bool) -> Option<SeedRange> {
        if self.overlaps_with(other) {
            Some(SeedRange {
                start: self.start.min(other.start),
                end: self.end.max(other.end),
                updated: updated,
            })
        } else {
            None
        }
    }

    fn overlaps_on_left_of(&self, range_start: usize, range_end: usize) -> bool {
        self.start < range_start && self.end >= range_start && self.end < range_end
    }

    fn overlaps_on_right_of(&self, range_start: usize, range_end: usize) -> bool {
        self.start <= range_end && self.end > range_end && self.start > range_start
    }

    fn overlaps_inside(&self, range_start: usize, range_end: usize) -> bool {
        self.start >= range_start && self.end <= range_end
    }

    fn overlaps_outside(&self, range_start: usize, range_end: usize) -> bool {
        self.start <= range_start && self.end >= range_end
    }

    fn map_to_new_range_and_remove_if_unupdated(
        &self,
        dest_range_start: usize,
        source_range_start: usize,
        range_length: usize,
    ) -> Option<(SeedRange, Option<SeedRange>, Option<SeedRange>)> {
        if !self.updated {
            let source_range_end = source_range_start + range_length - 1;
            let dest_range_end = dest_range_start + range_length - 1;
            if self.overlaps_on_left_of(source_range_start, source_range_end) {
                Some((
                    SeedRange {
                        start: dest_range_start,
                        end: dest_range_start + self.end - source_range_start,
                        updated: true,
                    },
                    Some(SeedRange {
                        start: self.start,
                        end: source_range_start - 1,
                        updated: false,
                    }),
                    None,
                ))
            } else if self.overlaps_on_right_of(source_range_start, source_range_end) {
                Some((
                    SeedRange {
                        start: dest_range_start + self.start - source_range_start,
                        end: dest_range_end,
                        updated: true,
                    },
                    Some(SeedRange {
                        start: source_range_end + 1,
                        end: self.end,
                        updated: false,
                    }),
                    None,
                ))
            } else if self.overlaps_inside(source_range_start, source_range_end) {
                Some((
                    SeedRange {
                        start: dest_range_start + self.start - source_range_start,
                        end: dest_range_end + self.end - source_range_end,
                        updated: true,
                    },
                    None,
                    None,
                ))
            } else if self.overlaps_outside(source_range_start, source_range_end) {
                let (first_arg, second_arg) = if self.start < source_range_end {
                    (
                        Some(SeedRange {
                            start: self.start,
                            end: source_range_start - 1,
                            updated: false,
                        }),
                        if self.end > source_range_end {
                            Some(SeedRange {
                                start: source_range_end + 1,
                                end: self.end,
                                updated: false,
                            })
                        } else {
                            None
                        },
                    )
                } else if self.end > source_range_end {
                    (
                        Some(SeedRange {
                            start: source_range_end + 1,
                            end: self.end,
                            updated: false,
                        }),
                        None,
                    )
                } else {
                    (None, None)
                };
                Some((
                    SeedRange {
                        start: dest_range_start,
                        end: dest_range_end,
                        updated: true,
                    },
                    first_arg,
                    second_arg,
                ))
            } else {
                return None;
            }
        } else {
            None
        }
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

    let mut data = get_seed_ranges(&buffer)?;
    join_ranges(&mut data);
    buffer.clear();

    while reader.read_line(&mut buffer)? != 0 {
        if !buffer.starts_with(|ch: char| ch.is_ascii_digit()) {
            if buffer.starts_with(|ch: char| ch.is_ascii_whitespace()) {
                clean_up(&mut data);
            }
        } else {
            let mut iter = buffer.split_ascii_whitespace();
            let dest_range_start = get_next_number_after_split(&mut iter)?;
            let source_range_start = get_next_number_after_split(&mut iter)?;
            let range_length = get_next_number_after_split(&mut iter)?;

            let mut new_ranges = Vec::new();
            let mut old_range_indices = Vec::new();
            let mut idx = 0;
            for range in &mut data {
                if let Some(mapping_data) = range.map_to_new_range_and_remove_if_unupdated(
                    dest_range_start,
                    source_range_start,
                    range_length,
                ) {
                    new_ranges.push(mapping_data.0);
                    if let Some(first_range_after_removing) = mapping_data.1 {
                        range.start = first_range_after_removing.start;
                        range.end = first_range_after_removing.end;
                    } else {
                        old_range_indices.push(idx);
                    }
                    if let Some(second_range_after_removing) = mapping_data.2 {
                        new_ranges.push(second_range_after_removing);
                    }
                }
                idx += 1;
            }
            for new_range in new_ranges {
                data.push(new_range);
            }
            for old_range_idx in old_range_indices {
                data.swap_remove(old_range_idx);
            }
        }
        buffer.clear();
    }

    join_ranges(&mut data);
    println!(
        "{}",
        data.first()
            .ok_or(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Empty data Vec"
            ))?
            .start
    );

    Ok(())
}

fn get_seed_ranges(entry: &str) -> io::Result<Vec<SeedRange>> {
    let mut data = Vec::new();
    let mut last_num = None;

    for val in entry.split_ascii_whitespace().skip(1) {
        let new_num = usize::from_str_radix(val, 10)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
        match last_num {
            None => last_num = Some(new_num),
            Some(num) => {
                data.push(SeedRange {
                    start: num,
                    end: num + new_num - 1,
                    updated: true,
                });
                last_num = None;
            }
        }
    }

    Ok(data)
}

fn clean_up(ranges: &mut Vec<SeedRange>) {
    join_ranges(ranges);
    for range in ranges {
        range.updated = false;
    }
}

fn join_ranges(ranges: &mut Vec<SeedRange>) {
    ranges.sort_unstable();
    for idx in (1..ranges.len()).rev() {
        if let Some(new_range) = ranges[idx].join_if_overlapping(&ranges[idx - 1], false) {
            ranges[idx - 1] = new_range;
            ranges.remove(idx);
        }
    }
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
