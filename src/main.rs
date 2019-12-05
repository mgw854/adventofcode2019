use std::error::Error;

mod inputhandling;
mod intcode;
mod day4;
mod fancyiters;

fn main() -> Result<(), Box<dyn Error>> {
  //let mut input : Vec<Vec<day3::WiringInstruction>> = inputhandling::parse_input_per_line(3, |s| day3::WiringInstruction::parse(s).map_err(|e| e.into()))?;

  let total = (108457..=562041).map(|i| i.to_string()).filter(|i| day4::adjacent_are_same(i)).filter(|i| day4::adjacent_never_decrease(i)).count();

  println!("There are {} total passwords in the search space", total);

  Ok(())
}
