use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

pub fn parse_input<T>(day: u8, parser: fn(&str) -> Result<T, Box<Error>>) -> Result<Vec<T>, Box<Error>>
{
    let mut file = File::open(format!("F:\\Development Ideas\\adventofcode2019\\src\\day{}\\input.txt", day))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    contents.lines().map(|s| s.trim()).map(|s| parser(s)).collect()
}
