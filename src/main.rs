use std::error::Error;

mod day6;
mod fancyiters;
mod inputhandling;
mod intcode;

fn main() -> Result<(), Box<dyn Error>> {
  let flat_directions: Vec<day6::OrbitalDirection> =
    inputhandling::parse_input_per_line(6, |s| day6::OrbitalDirection::parse(s).map_err(|e| e.into()))?;

    let graph = day6::generate_map(&flat_directions);
   
    println!("The total number of orbits is {}", day6::calculate_orbits(&graph));

  Ok(())
}

