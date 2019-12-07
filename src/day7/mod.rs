pub struct Amplifier {
  pub phase_setting: u8, // Bound 0-4
  pub input_signal: i32
}

pub fn phase_setting_generator() -> Vec<Vec<u8>> {
  let mut results = Vec::new();
  
  for i in 0..=4 {
    for j in 0..=4 {
      if i == j { continue; }

      for k in 0..=4 {
        if i == k || j == k { continue; }

        for l in 0..=4 {
          if i == l || j == l || k == l { continue; }

          for m in 0..=4 {
            if i == m || j == m || k == m || l == m { continue; }

            results.push(vec![i, j, k, l, m]);
          }
        }
      }
    }
  }

  results
}

use super::intcode::Intcode;

impl Amplifier {
  pub fn run(&self, mut processor: Intcode) -> i32 {
    processor.input = vec![self.phase_setting as i32, self.input_signal];
    let cpu = processor.process();
    cpu.0.read_output()[0]
  }
}

#[cfg(test)]
mod tests {
    use super::*;

  fn parse_csv(input: &str) -> Vec<i32> {
    input
        .split(",")
        .map(|s| s.trim())
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
  }

  #[test]
  fn given_input_part1_1() { 
    let instructions = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
    let mut cpu0 = Intcode::create(parse_csv(instructions));
    let mut cpu1 = Intcode::create(parse_csv(instructions));
    let mut cpu2 = Intcode::create(parse_csv(instructions));
    let mut cpu3 = Intcode::create(parse_csv(instructions));
    let mut cpu4 = Intcode::create(parse_csv(instructions));

    let amp0 = Amplifier { phase_setting: 4, input_signal: 0 };
    let amp1 = Amplifier { phase_setting: 3, input_signal: amp0.run(cpu0) };
    let amp2 = Amplifier { phase_setting: 2, input_signal: amp1.run(cpu1) };
    let amp3 = Amplifier { phase_setting: 1, input_signal: amp2.run(cpu2) };
    let amp4 = Amplifier { phase_setting: 0, input_signal: amp3.run(cpu3) };

    assert_eq!(amp4.run(cpu4), 43210);
  }

  #[test]
  fn given_input_part1_2() { 
    let instructions = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";
    let mut cpu0 = Intcode::create(parse_csv(instructions));
    let mut cpu1 = Intcode::create(parse_csv(instructions));
    let mut cpu2 = Intcode::create(parse_csv(instructions));
    let mut cpu3 = Intcode::create(parse_csv(instructions));
    let mut cpu4 = Intcode::create(parse_csv(instructions));

    let amp0 = Amplifier { phase_setting: 0, input_signal: 0 };
    let amp1 = Amplifier { phase_setting: 1, input_signal: amp0.run(cpu0) };
    let amp2 = Amplifier { phase_setting: 2, input_signal: amp1.run(cpu1) };
    let amp3 = Amplifier { phase_setting: 3, input_signal: amp2.run(cpu2) };
    let amp4 = Amplifier { phase_setting: 4, input_signal: amp3.run(cpu3) };

    assert_eq!(amp4.run(cpu4), 54321);
  }

  #[test]
  fn given_input_part1_3() { 
    let instructions = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";
    let mut cpu0 = Intcode::create(parse_csv(instructions));
    let mut cpu1 = Intcode::create(parse_csv(instructions));
    let mut cpu2 = Intcode::create(parse_csv(instructions));
    let mut cpu3 = Intcode::create(parse_csv(instructions));
    let mut cpu4 = Intcode::create(parse_csv(instructions));

    let amp0 = Amplifier { phase_setting: 1, input_signal: 0 };
    let amp1 = Amplifier { phase_setting: 0, input_signal: amp0.run(cpu0) };
    let amp2 = Amplifier { phase_setting: 4, input_signal: amp1.run(cpu1) };
    let amp3 = Amplifier { phase_setting: 3, input_signal: amp2.run(cpu2) };
    let amp4 = Amplifier { phase_setting: 2, input_signal: amp3.run(cpu3) };

    assert_eq!(amp4.run(cpu4), 65210);
  }}