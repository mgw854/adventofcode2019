use super::intcode_8086::{Intcode8086};
use bus::BusReader;

pub struct Amplifier {
  processor: Intcode8086,
  pub output: BusReader<i32>
}

impl Amplifier {
  pub fn create(phase_setting: u8, input_signal: i32, mut cpu: Intcode8086, mut input: BusReader<i32>) -> Amplifier {
    let cpu_input : crossbeam_channel::Sender<i32> = cpu.get_input_port();
    cpu_input.send(phase_setting as i32).expect("Sending a phase signal should not fail");
    cpu_input.send(input_signal).expect("Sending an input signal should not fail");

    std::thread::spawn(move || {
      loop {
        match input.recv() {
          Ok(x) => match cpu_input.send(x) { Err(e) => break, _ => continue },
          Err(e) => { break; }
        };
      }
    });

    let output = cpu.get_output_port();

    Amplifier {
      processor: cpu,
      output: output
    }
  }

  pub fn create_no_value(phase_setting: u8, mut cpu: Intcode8086, mut input: BusReader<i32>) -> Amplifier {
    let cpu_input : crossbeam_channel::Sender<i32> = cpu.get_input_port();
    cpu_input.send(phase_setting as i32).expect("Sending a phase signal should not fail");

    std::thread::spawn(move || {
      loop {
        match input.recv() {
          Ok(x) => cpu_input.send(x).expect("Send shouldn't fail (nv)"),
          Err(e) => { break; }
        };
      }
    });

    let output = cpu.get_output_port();

    Amplifier {
      processor: cpu,
      output: output
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

impl Amplifier {
  pub fn run(self) -> std::thread::JoinHandle<Intcode8086> {
    self.processor.process()
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
    let mut cpu0 = Intcode8086::initialize(parse_csv(instructions));
    let mut cpu1 = Intcode8086::initialize(parse_csv(instructions));
    let mut cpu2 = Intcode8086::initialize(parse_csv(instructions));
    let mut cpu3 = Intcode8086::initialize(parse_csv(instructions));
    let mut cpu4 = Intcode8086::initialize(parse_csv(instructions));

    let io0 = cpu0.get_output_port();
    let io1 = cpu1.get_output_port();
    let io2 = cpu2.get_output_port();
    let io3 = cpu3.get_output_port();
    let io4 = cpu4.get_output_port();

    let mut output : BusReader<i32> = cpu4.get_output_port();

    let amp0 = Amplifier::create(4, 0, cpu0, io4);
    let amp1 = Amplifier::create_no_value(3, cpu1, io0);
    let amp2 = Amplifier::create_no_value(2, cpu2, io1);
    let amp3 = Amplifier::create_no_value(1, cpu3, io2);
    let amp4 = Amplifier::create_no_value(0, cpu4, io3);

    amp0.run();
    amp1.run();
    amp2.run();
    amp3.run();
    amp4.run().join();

    let mut max = 0;

    loop {
      match output.recv() {
        Ok(v) => max = v,
        Err(e) => break
      };
    }
    
    assert_eq!(max, 43210);
  }

  #[test]
  fn given_input_part1_2() { 
    let instructions = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";
    let mut cpu0 = Intcode8086::initialize(parse_csv(instructions));
    let mut cpu1 = Intcode8086::initialize(parse_csv(instructions));
    let mut cpu2 = Intcode8086::initialize(parse_csv(instructions));
    let mut cpu3 = Intcode8086::initialize(parse_csv(instructions));
    let mut cpu4 = Intcode8086::initialize(parse_csv(instructions));

    let io0 = cpu0.get_output_port();
    let io1 = cpu1.get_output_port();
    let io2 = cpu2.get_output_port();
    let io3 = cpu3.get_output_port();
    let io4 = cpu4.get_output_port();

    let mut output : BusReader<i32> = cpu4.get_output_port();

    let amp0 = Amplifier::create(0, 0, cpu0, io4);
    let amp1 = Amplifier::create_no_value(1, cpu1, io0);
    let amp2 = Amplifier::create_no_value(2, cpu2, io1);
    let amp3 = Amplifier::create_no_value(3, cpu3, io2);
    let amp4 = Amplifier::create_no_value(4, cpu4, io3);

    amp0.run();
    amp1.run();
    amp2.run();
    amp3.run();
    amp4.run().join();

    let mut max = 0;

    loop {
      match output.recv() {
        Ok(v) => max = v,
        Err(e) => break
      };
    }
    
    assert_eq!(max, 54321);
  }

  #[test]
  fn given_input_part1_3() { 
    let instructions = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";
    let mut cpu0 = Intcode8086::initialize(parse_csv(instructions));
    let mut cpu1 = Intcode8086::initialize(parse_csv(instructions));
    let mut cpu2 = Intcode8086::initialize(parse_csv(instructions));
    let mut cpu3 = Intcode8086::initialize(parse_csv(instructions));
    let mut cpu4 = Intcode8086::initialize(parse_csv(instructions));

    let io0 = cpu0.get_output_port();
    let io1 = cpu1.get_output_port();
    let io2 = cpu2.get_output_port();
    let io3 = cpu3.get_output_port();
    let io4 = cpu4.get_output_port();

    let mut output : BusReader<i32> = cpu4.get_output_port();

    let amp0 = Amplifier::create(1, 0, cpu0, io4);
    let amp1 = Amplifier::create_no_value(0, cpu1, io0);
    let amp2 = Amplifier::create_no_value(4, cpu2, io1);
    let amp3 = Amplifier::create_no_value(3, cpu3, io2);
    let amp4 = Amplifier::create_no_value(2, cpu4, io3);

    amp0.run();
    amp1.run();
    amp2.run();
    amp3.run();
    amp4.run().join();

    let mut max = 0;

    loop {
      match output.recv() {
        Ok(v) => max = v,
        Err(e) => break
      };
    }
    
    assert_eq!(max, 65210);
  }

  #[test]
  fn given_input_part2_1() { 
    let instructions = "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
    let mut cpu0 = Intcode8086::initialize(parse_csv(instructions));
    let mut cpu1 = Intcode8086::initialize(parse_csv(instructions));
    let mut cpu2 = Intcode8086::initialize(parse_csv(instructions));
    let mut cpu3 = Intcode8086::initialize(parse_csv(instructions));
    let mut cpu4 = Intcode8086::initialize(parse_csv(instructions));

    let io0 = cpu0.get_output_port();
    let io1 = cpu1.get_output_port();
    let io2 = cpu2.get_output_port();
    let io3 = cpu3.get_output_port();
    let io4 = cpu4.get_output_port();

    let mut output : BusReader<i32> = cpu4.get_output_port();

    let amp0 = Amplifier::create(9, 0, cpu0, io4);
    let amp1 = Amplifier::create_no_value(8, cpu1, io0);
    let amp2 = Amplifier::create_no_value(7, cpu2, io1);
    let amp3 = Amplifier::create_no_value(6, cpu3, io2);
    let amp4 = Amplifier::create_no_value(5, cpu4, io3);

    amp0.run();
    amp1.run();
    amp2.run();
    amp3.run();
    amp4.run().join();

    let mut max = 0;

    loop {
      match output.recv() {
        Ok(v) => max = v,
        Err(e) => break
      };
    }
    
    assert_eq!(max, 139629729);
  }

  
  #[test]
  fn given_input_part2_2() { 
    let instructions = "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10";
    let mut cpu0 = Intcode8086::initialize(parse_csv(instructions));
    let mut cpu1 = Intcode8086::initialize(parse_csv(instructions));
    let mut cpu2 = Intcode8086::initialize(parse_csv(instructions));
    let mut cpu3 = Intcode8086::initialize(parse_csv(instructions));
    let mut cpu4 = Intcode8086::initialize(parse_csv(instructions));

    let io0 = cpu0.get_output_port();
    let io1 = cpu1.get_output_port();
    let io2 = cpu2.get_output_port();
    let io3 = cpu3.get_output_port();
    let io4 = cpu4.get_output_port();

    let mut output : BusReader<i32> = cpu4.get_output_port();

    let amp0 = Amplifier::create(9, 0, cpu0, io4);
    let amp1 = Amplifier::create_no_value(7, cpu1, io0);
    let amp2 = Amplifier::create_no_value(8, cpu2, io1);
    let amp3 = Amplifier::create_no_value(5, cpu3, io2);
    let amp4 = Amplifier::create_no_value(6, cpu4, io3);

    amp0.run();
    amp1.run();
    amp2.run();
    amp3.run();
    amp4.run().join();

    let mut max = 0;

    loop {
      match output.recv() {
        Ok(v) => max = v,
        Err(e) => break
      };
    }
    
    assert_eq!(max, 18216);
  }
}