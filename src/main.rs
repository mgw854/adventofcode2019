use std::error::Error;

mod inputhandling;
mod intcode;
mod day3;

fn main() -> Result<(), Box<dyn Error>> {
  let mut input : Vec<Vec<day3::WiringInstruction>> = inputhandling::parse_input_per_line(3, |s| day3::WiringInstruction::parse(s).map_err(|e| e.into()))?;

  let one = day3::generate_points(&input[0]);
  let two = day3::generate_points(&input[1]);

  let distance = day3::generate_manhattan_distance(&one, &two);

  println!("The Manhattan distance is {}", distance);

  Ok(())
}
