use std::error::Error;

mod day7;
mod fancyiters;
mod inputhandling;
mod intcode;

fn main() -> Result<(), Box<dyn Error>> {
  let vonNeumann : Vec<i32> = inputhandling::parse_csv_input(7, |s| s.parse::<i32>().map_err(|e| e.into()))?;

  let mut max = 0;
/*
  for sequence in day7::phase_setting_generator() {
    let mut cpu0 = intcode::Intcode::create(vonNeumann.clone());
    let mut cpu1 = intcode::Intcode::create(vonNeumann.clone());
    let mut cpu2 = intcode::Intcode::create(vonNeumann.clone());
    let mut cpu3 = intcode::Intcode::create(vonNeumann.clone());
    let mut cpu4 = intcode::Intcode::create(vonNeumann.clone());

    let mut amp0 = day7::Amplifier::create(sequence[0], 0);
    let mut amp1 = day7::Amplifier::create(sequence[1], amp0.run(cpu0));
    let mut amp2 = day7::Amplifier::create(sequence[2], amp1.run(cpu1));
    let mut amp3 = day7::Amplifier::create(sequence[3], amp2.run(cpu2));
    let mut amp4 = day7::Amplifier::create(sequence[4], amp3.run(cpu3));
    let final_ouput = amp4.run(cpu4);

    if final_ouput > max {
      max = final_ouput;
    }
  }
*/
  println!("The maximum output is {}", max);

  Ok(())
}

