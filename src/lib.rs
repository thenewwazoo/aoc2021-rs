use std::fs::File;
use std::io::Error;
use std::io::{BufRead, BufReader, Lines};
use std::str::FromStr;

pub mod sub;

pub fn read_lines_from(input_fn: &str) -> Result<Lines<BufReader<File>>, Error> {
    Ok(BufReader::new(File::open(input_fn)?).lines())
}

pub fn lines_as_vec(input_fn: &str) -> Vec<String> {
    read_lines_from(input_fn)
        .unwrap() // die if we can't read the file
        .collect::<Result<Vec<String>, std::io::Error>>()
        .unwrap()
}

pub fn str_as_vec(input: &str) -> Vec<String> {
    input.lines().map(str::to_string).collect::<Vec<String>>()
}

pub fn get_num_list<T>(input: &[String]) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    input[0]
        .split(',')
        .map(|s| s.parse::<T>().unwrap())
        .collect()
}
