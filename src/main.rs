use std::error::Error;

mod day7;
mod fancyiters;
mod inputhandling;
mod intcode;
mod intcode_8086;

fn main() -> Result<(), Box<dyn Error>> {
  let vonNeumann : Vec<i32> = inputhandling::parse_csv_input(7, |s| s.parse::<i32>().map_err(|e| e.into()))?;

  let mut max = 0;

  for sequence in day7::phase_setting_generator() {
    let cpu0 = intcode::Intcode::create(vonNeumann.clone());
    let cpu1 = intcode::Intcode::create(vonNeumann.clone());
    let cpu2 = intcode::Intcode::create(vonNeumann.clone());
    let cpu3 = intcode::Intcode::create(vonNeumann.clone());
    let cpu4 = intcode::Intcode::create(vonNeumann.clone());

    let mut amp0 = day7::Amplifier::create(sequence[0], 0, cpu0);
    let mut amp1 = day7::Amplifier::create_no_value(sequence[1], cpu1);
    let mut amp2 = day7::Amplifier::create_no_value(sequence[2], cpu2);
    let mut amp3 = day7::Amplifier::create_no_value(sequence[3], cpu3);
    let mut amp4 = day7::Amplifier::create_no_value(sequence[4], cpu4);

    let mut halted = false;

    while !halted {
      amp1.input_signal = match amp0.run() {
        day7::AmplifierState::Halt { value: x } => {halted = true; x}
        day7::AmplifierState::Hot { value: x } => x 
      };

      amp2.input_signal = match amp1.run() {
        day7::AmplifierState::Halt { value: x } => {halted = true; x}
        day7::AmplifierState::Hot { value: x } => x 
      };

      amp3.input_signal = match amp2.run() {
        day7::AmplifierState::Halt { value: x } => {halted = true; x}
        day7::AmplifierState::Hot { value: x } => x 
      };

      amp4.input_signal = match amp3.run() {
        day7::AmplifierState::Halt { value: x } => {halted = true; x}
        day7::AmplifierState::Hot { value: x } => x 
      };

      amp0.input_signal = match amp4.run() {
        day7::AmplifierState::Halt { value: x } => { halted = true; x },
        day7::AmplifierState::Hot { value: x } => x 
      };
    }

    if amp0.input_signal > max {
      max = amp0.input_signal;
    }
  }

  println!("The maximum output is {}", max);

  Ok(())
}

