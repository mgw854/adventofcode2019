use std::error::Error;

mod fancyiters;
mod inputhandling;
mod intcode_8086;

fn main() -> Result<(), Box<dyn Error>> {
  let vonNeumann : Vec<i64> = inputhandling::parse_csv_input(9, |s| s.parse::<i64>().map_err(|e| e.into()))?;

  let mut cpu = intcode_8086::Intcode8086::initialize(vonNeumann);
  cpu.get_input_port().send(1).expect("Test input");
  let mut io = cpu.get_output_port();

  let handle = cpu.process();

  handle.join().expect("");

  for v in io.iter() {
    println!("{}", v);
  }

  Ok(())
}

