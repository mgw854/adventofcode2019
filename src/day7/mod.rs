use super::intcode::{Intcode, IntcodeState};

pub struct Amplifier {
  pub phase_setting: u8, // Bound 0-4 or 5-9
  pub input_signal: i32,
  started: bool,
  processor: Intcode
}

impl Amplifier {
  pub fn create(phase_setting: u8, input_signal: i32, cpu: Intcode) -> Amplifier {
    Amplifier {
      phase_setting: phase_setting,
      input_signal: input_signal,
      started: false,
      processor: cpu
    }
  }

  pub fn create_no_value(phase_setting: u8, cpu: Intcode) -> Amplifier {
    Amplifier {
      phase_setting: phase_setting,
      input_signal: 0,
      started: false,
      processor: cpu
    }
  }
}

pub fn phase_setting_generator() -> Vec<Vec<u8>> {
  let mut results = Vec::new();
  
  for i in 5..=9 {
    for j in 5..=9 {
      if i == j { continue; }

      for k in 5..=9 {
        if i == k || j == k { continue; }

        for l in 5..=9 {
          if i == l || j == l || k == l { continue; }

          for m in 5..=9 {
            if i == m || j == m || k == m || l == m { continue; }

            results.push(vec![i, j, k, l, m]);
          }
        }
      }
    }
  }

  results
}

pub enum AmplifierState {
  Hot { value: i32 },
  Halt { value: i32 }
}

impl Amplifier {
  pub fn run(&mut self) -> AmplifierState {
    if self.started == false {
      self.processor.push_input(self.phase_setting as i32);
      self.started = true;
    }

    self.processor.push_input(self.input_signal);

    match self.processor.process() {
      IntcodeState::IOWait => {
        let output = self.processor.read_output().unwrap().clone();
        AmplifierState::Hot { value: output }
      },
      IntcodeState::Halt { first_value: _ } => AmplifierState::Halt { value: self.processor.read_output().unwrap().clone() }
    }
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

  impl AmplifierState {
    fn expect(&self) -> i32 {
      match self {
        AmplifierState::Halt { value: x } => *x,
        AmplifierState::Hot { value: x } => *x
    }
  }
}

  #[test]
  fn given_input_part1_1() { 
    let instructions = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
    let cpu0 = Intcode::create(parse_csv(instructions));
    let cpu1 = Intcode::create(parse_csv(instructions));
    let cpu2 = Intcode::create(parse_csv(instructions));
    let cpu3 = Intcode::create(parse_csv(instructions));
    let cpu4 = Intcode::create(parse_csv(instructions));

    let mut amp0 = Amplifier::create(4, 0, cpu0);
    let mut amp1 = Amplifier::create(3, amp0.run().expect(), cpu1);
    let mut amp2 = Amplifier::create(2, amp1.run().expect(), cpu2);
    let mut amp3 = Amplifier::create(1, amp2.run().expect(), cpu3);
    let mut amp4 = Amplifier::create(0, amp3.run().expect(), cpu4);

    assert_eq!(amp4.run().expect(), 43210);
  }

  #[test]
  fn given_input_part1_2() { 
    let instructions = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";
    let cpu0 = Intcode::create(parse_csv(instructions));
    let cpu1 = Intcode::create(parse_csv(instructions));
    let cpu2 = Intcode::create(parse_csv(instructions));
    let cpu3 = Intcode::create(parse_csv(instructions));
    let cpu4 = Intcode::create(parse_csv(instructions));

    let mut amp0 = Amplifier::create(0, 0, cpu0);
    let mut amp1 = Amplifier::create(1, amp0.run().expect(), cpu1);
    let mut amp2 = Amplifier::create(2, amp1.run().expect(), cpu2);
    let mut amp3 = Amplifier::create(3, amp2.run().expect(), cpu3);
    let mut amp4 = Amplifier::create(4, amp3.run().expect(), cpu4);

    assert_eq!(amp4.run().expect(), 54321);
  }

  #[test]
  fn given_input_part1_3() { 
    let instructions = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";
    let cpu0 = Intcode::create(parse_csv(instructions));
    let cpu1 = Intcode::create(parse_csv(instructions));
    let cpu2 = Intcode::create(parse_csv(instructions));
    let cpu3 = Intcode::create(parse_csv(instructions));
    let cpu4 = Intcode::create(parse_csv(instructions));

    let mut amp0 = Amplifier::create(1, 0, cpu0);
    let mut amp1 = Amplifier::create(0, amp0.run().expect(), cpu1);
    let mut amp2 = Amplifier::create(4, amp1.run().expect(), cpu2);
    let mut amp3 = Amplifier::create(3, amp2.run().expect(), cpu3);
    let mut amp4 = Amplifier::create(2, amp3.run().expect(), cpu4);

    assert_eq!(amp4.run().expect(), 65210);
  }

  #[test]
  fn given_input_part2_1() { 
    let instructions = "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
    let cpu0 = Intcode::create(parse_csv(instructions));
    let cpu1 = Intcode::create(parse_csv(instructions));
    let cpu2 = Intcode::create(parse_csv(instructions));
    let cpu3 = Intcode::create(parse_csv(instructions));
    let cpu4 = Intcode::create(parse_csv(instructions));

    let mut amp0 = Amplifier::create(9, 0, cpu0);
    let mut amp1 = Amplifier::create_no_value(8, cpu1);
    let mut amp2 = Amplifier::create_no_value(7, cpu2);
    let mut amp3 = Amplifier::create_no_value(6, cpu3);
    let mut amp4 = Amplifier::create_no_value(5, cpu4);

    let mut halted = false;

    while !halted {
      amp1.input_signal = match amp0.run() {
        AmplifierState::Halt { value: x } => {halted = true; x}
        AmplifierState::Hot { value: x } => x 
      };

      amp2.input_signal = match amp1.run() {
        AmplifierState::Halt { value: x } => {halted = true; x}
        AmplifierState::Hot { value: x } => x 
      };

      amp3.input_signal = match amp2.run() {
        AmplifierState::Halt { value: x } => {halted = true; x}
        AmplifierState::Hot { value: x } => x 
      };

      amp4.input_signal = match amp3.run() {
        AmplifierState::Halt { value: x } => {halted = true; x}
        AmplifierState::Hot { value: x } => x 
      };

      amp0.input_signal = match amp4.run() {
        AmplifierState::Halt { value: x } => { assert_eq!(x, 139629729); return; },
        AmplifierState::Hot { value: x } => x 
      };
    }

    assert_eq!(amp4.run().expect(), 139629729);
  }

  
  #[test]
  fn given_input_part2_2() { 
    let instructions = "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10";
    let cpu0 = Intcode::create(parse_csv(instructions));
    let cpu1 = Intcode::create(parse_csv(instructions));
    let cpu2 = Intcode::create(parse_csv(instructions));
    let cpu3 = Intcode::create(parse_csv(instructions));
    let cpu4 = Intcode::create(parse_csv(instructions));

    let mut amp0 = Amplifier::create(9, 0, cpu0);
    let mut amp1 = Amplifier::create_no_value(7, cpu1);
    let mut amp2 = Amplifier::create_no_value(8, cpu2);
    let mut amp3 = Amplifier::create_no_value(5, cpu3);
    let mut amp4 = Amplifier::create_no_value(6, cpu4);

    let mut halted = false;

    while !halted {
      amp1.input_signal = match amp0.run() {
        AmplifierState::Halt { value: x } => {halted = true; x}
        AmplifierState::Hot { value: x } => x 
      };

      amp2.input_signal = match amp1.run() {
        AmplifierState::Halt { value: x } => {halted = true; x}
        AmplifierState::Hot { value: x } => x 
      };

      amp3.input_signal = match amp2.run() {
        AmplifierState::Halt { value: x } => {halted = true; x}
        AmplifierState::Hot { value: x } => x 
      };

      amp4.input_signal = match amp3.run() {
        AmplifierState::Halt { value: x } => {halted = true; x}
        AmplifierState::Hot { value: x } => x 
      };

      amp0.input_signal = match amp4.run() {
        AmplifierState::Halt { value: x } => { assert_eq!(x, 18216); return; },
        AmplifierState::Hot { value: x } => x 
      };
    }

    assert_eq!(amp4.run().expect(), 18216);
  }
}