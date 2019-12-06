use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn parse_input<T>(
    day: u8,
    parser: fn(&str) -> Result<T, Box<dyn Error>>,
) -> Result<Vec<T>, Box<dyn Error>> {
    let pathstr = format!(".\\src\\day{}\\input.txt", day);
    let path = Path::new(&pathstr);
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    contents
        .lines()
        .map(|s| s.trim())
        .map(|s| parser(s))
        .collect()
}

pub fn parse_input_per_line<T>(
    day: u8,
    parser: fn(&str) -> Result<T, Box<dyn Error>>,
) -> Result<Vec<Vec<T>>, Box<dyn Error>> {
    let pathstr = format!(".\\src\\day{}\\input.txt", day);
    let path = Path::new(&pathstr);
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    contents
        .lines()
        .map(|s| s.trim().split(",").map(|x| parser(x)).collect())
        .collect()
}

pub fn parse_csv_input<T>(
    day: u8,
    parser: fn(&str) -> Result<T, Box<dyn Error>>,
) -> Result<Vec<T>, Box<dyn Error>> {
    let pathstr = format!(".\\src\\day{}\\input.txt", day);
    let path = Path::new(&pathstr);
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    contents
        .split(",")
        .map(|s| s.trim())
        .map(|s| parser(s))
        .collect()
}
