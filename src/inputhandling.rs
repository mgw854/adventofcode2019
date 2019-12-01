use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::path::Path;

pub fn parse_input<T>(day: u8, parser: fn(&str) -> Result<T, Box<Error>>) -> Result<Vec<T>, Box<Error>>
{
    let pathstr = format!(".\\src\\day{}\\input.txt", day);
    let path = Path::new(&pathstr);
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    contents.lines().map(|s| s.trim()).map(|s| parser(s)).collect()
}
