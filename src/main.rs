use std::error::Error;

mod inputhandling;

fn main() -> Result<(), Box<Error>> {
    let input : Vec<u32> = inputhandling::parse_input(1, |s| s.parse::<u32>().map_err(|e| e.into()))?;
    Ok(())
}
