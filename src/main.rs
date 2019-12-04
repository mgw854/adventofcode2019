use std::error::Error;

mod inputhandling;
mod intcode;
mod day4;

fn main() -> Result<(), Box<dyn Error>> {
  //let mut input : Vec<Vec<day3::WiringInstruction>> = inputhandling::parse_input_per_line(3, |s| day3::WiringInstruction::parse(s).map_err(|e| e.into()))?;



  Ok(())
}
