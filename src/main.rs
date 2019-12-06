use std::error::Error;

mod day4;
mod fancyiters;
mod inputhandling;
mod intcode;

fn main() -> Result<(), Box<dyn Error>> {
    let vonNeumann: Vec<i32> =
        inputhandling::parse_csv_input(5, |s| s.parse::<i32>().map_err(|e| e.into()))?;

    let mut cpu = intcode::Intcode::create(vonNeumann);
    cpu.input = 5;

    let cpu = cpu.process().0;

    dbg!(cpu.read_output());

    Ok(())
}
