use std::error::Error;

mod day6;
mod day7;
mod fancyiters;
mod inputhandling;
mod intcode;

fn main() -> Result<(), Box<dyn Error>> {
  let vonNeumann : Vec<i32> = inputhandling::parse_csv_input(7, |s| s.parse::<i32>().map_err(|e| e.into()))?;

  let mut max = 0;

  for sequence in day7::phase_setting_generator() {
    let mut cpu0 = intcode::Intcode::create(vonNeumann.clone());
    let mut cpu1 = intcode::Intcode::create(vonNeumann.clone());
    let mut cpu2 = intcode::Intcode::create(vonNeumann.clone());
    let mut cpu3 = intcode::Intcode::create(vonNeumann.clone());
    let mut cpu4 = intcode::Intcode::create(vonNeumann.clone());

    let amp0 = day7::Amplifier { phase_setting: sequence[0], input_signal: 0 };
    let amp1 = day7::Amplifier { phase_setting: sequence[1], input_signal: amp0.run(cpu0) };
    let amp2 = day7::Amplifier { phase_setting: sequence[2], input_signal: amp1.run(cpu1) };
    let amp3 = day7::Amplifier { phase_setting: sequence[3], input_signal: amp2.run(cpu2) };
    let amp4 = day7::Amplifier { phase_setting: sequence[4], input_signal: amp3.run(cpu3) };
    let final_ouput = amp4.run(cpu4);

    if final_ouput > max {
      max = final_ouput;
    }
  }

  println!("The maximum output is {}", max);

  Ok(())
}

