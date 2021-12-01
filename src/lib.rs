use std::io::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

pub fn read_lines_from(input_fn: &str) -> Result<Lines<BufReader<File>>, Error> {
    Ok(
        BufReader::new(File::open(input_fn)?).lines()
    )
}
