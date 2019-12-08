use std::thread;
use crossbeam_channel::{ Sender, Receiver, unbounded };

pub struct Intcode8086 {
  instruction_pointer: usize,
  von_neumann_tape: Vec<i32>,
  input_sender: Sender<i32>,
  input_receiver: Receiver<i32>,
  output_sender: Sender<i32>,
  output_receiver: Receiver<i32>
}

impl Intcode8086 {
  pub fn initialize(von_neumann_tape: Vec<i32>) -> Intcode8086 {
    let (i_s, i_r) = unbounded();
    let (o_s, o_r) = unbounded();

    Intcode8086 {
      instruction_pointer: 0,
      von_neumann_tape: von_neumann_tape,
      input_sender: i_s,
      input_receiver: i_r,
      output_sender: o_s,
      output_receiver: o_r
    }
  }

  pub fn get_input_port(&self) -> Sender<i32> {
    self.input_sender.clone()
  }

  pub fn get_output_port(&self) -> Receiver<i32> {
    self.output_receiver.clone()
  }

  pub fn process(mut self) -> std::thread::JoinHandle<Self> {
    thread::spawn(move || {
      while self.instruction_pointer < self.von_neumann_tape.len() {
        let instruction = Intcode8086::decode_instruction(self.von_neumann_tape[self.instruction_pointer] as usize);

        let res = match instruction {
          Some(Instruction::Add(arg1, arg2)) => self.three_arg_fn(arg1, arg2, |a, b| a + b),
          Some(Instruction::Multiply(arg1, arg2)) => self.three_arg_fn(arg1, arg2, |a, b| a * b),
          Some(Instruction::StoreInput) => self.store_input(),
          Some(Instruction::WriteOutput(arg1)) => self.write_output(arg1),
          Some(Instruction::JumpIfTrue(arg1, arg2)) => self.jump(arg1, arg2, true),
          Some(Instruction::JumpIfFalse(arg1, arg2)) => self.jump(arg1, arg2, false),
          Some(Instruction::LessThan(arg1, arg2)) => self.compare_args(arg1, arg2, |a, b| a < b),
          Some(Instruction::Equals(arg1, arg2)) => self.compare_args(arg1, arg2, |a, b| a == b),
          Some(Instruction::Halt) => InstructionResult {
            next_instruction_pointer: None,
            store: None,
          },
          None => panic!("Unknown instruction")
        };

        match res.next_instruction_pointer {
          Some(x) => self.instruction_pointer = x,
          None => break,
        };

        if let Some(store) = res.store {
            self.von_neumann_tape[store.address] = store.value;
        }
      }

      self
    })
  }

  pub fn get_memory_at(&self, position: usize) -> i32 {
    self.von_neumann_tape[position]
  }

  fn decode_instruction(opcode: usize) -> Option<Instruction> {
    match opcode % 100 {
      1 => { let p = ParameterMode::parse(opcode, 2); Some(Instruction::Add(p[0], p[1])) }
      2 => { let p = ParameterMode::parse(opcode, 2); Some(Instruction::Multiply(p[0], p[1])) }
      3 => Some(Instruction::StoreInput),
      4 => { let p = ParameterMode::parse(opcode, 1); Some(Instruction::WriteOutput(p[0])) },
      5 => { let p = ParameterMode::parse(opcode, 2); Some(Instruction::JumpIfTrue(p[0], p[1])) },
      6 => { let p = ParameterMode::parse(opcode, 2); Some(Instruction::JumpIfFalse(p[0], p[1])) },
      7 => { let p = ParameterMode::parse(opcode, 2); Some(Instruction::LessThan(p[0], p[1])) },
      8 => { let p = ParameterMode::parse(opcode, 2); Some(Instruction::Equals(p[0], p[1])) },
      99 => Some(Instruction::Halt),
      _ => None,
    }
  }

  fn three_arg_fn(&self, arg1: ParameterMode, arg2: ParameterMode, func: fn(i32, i32) -> i32) -> InstructionResult {
    let store_address: usize = self.von_neumann_tape[self.instruction_pointer + 3] as usize;
    let store_value = func(arg1.get(self, 1), arg2.get(self, 2));

    InstructionResult {
        next_instruction_pointer: Some(self.instruction_pointer + 4),
        store: Some(StoreInstruction {
            address: store_address,
            value: store_value,
        }),
    }
  }

  fn jump(&self, arg1: ParameterMode, arg2: ParameterMode, jump_if: bool) -> InstructionResult {
    let eval = arg1.get(self, 1);
    let next = match jump_if {
      true => {
        if eval != 0 {
          arg2.get(self, 2) as usize
        } else {
          self.instruction_pointer + 3
        }
      }
      false => {
        if eval == 0 {
          arg2.get(self, 2) as usize
        } else {
          self.instruction_pointer + 3
        }
      }
    };

    InstructionResult {
      next_instruction_pointer: Some(next),
      store: None,
    }
  }

  fn compare_args(&self, arg1: ParameterMode, arg2: ParameterMode, func: fn(i32, i32) -> bool) -> InstructionResult {
    let store_address: usize = self.von_neumann_tape[self.instruction_pointer + 3] as usize;
    let result = func(arg1.get(self, 1), arg2.get(self, 2));

    InstructionResult {
      next_instruction_pointer: Some(self.instruction_pointer + 4),
      store: Some(StoreInstruction {
          address: store_address,
          value: if result { 1 } else { 0 },
      }),
    }
  }

  fn store_input(&self) -> InstructionResult {
    InstructionResult {
      next_instruction_pointer: Some(self.instruction_pointer + 2),
      store: Some(StoreInstruction {
        address: self.von_neumann_tape[self.instruction_pointer + 1] as usize,
        value: self.input_receiver.recv().unwrap()
      })
    }    
  }

  fn write_output(&self, arg1: ParameterMode) -> InstructionResult {
    self.output_sender.send(arg1.get(self, 1));
    InstructionResult {
      next_instruction_pointer: Some(self.instruction_pointer + 2),
      store: None,
    }
  }
}

enum Instruction {
  Add(ParameterMode, ParameterMode),
  Multiply(ParameterMode, ParameterMode),
  StoreInput,
  WriteOutput(ParameterMode),
  JumpIfTrue(ParameterMode, ParameterMode),
  JumpIfFalse(ParameterMode, ParameterMode),
  LessThan(ParameterMode, ParameterMode),
  Equals(ParameterMode, ParameterMode),

  Halt
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum ParameterMode {
  Position,
  Immediate,
}

impl ParameterMode {
  fn parse(opcode: usize, number_of_positions: usize) -> Vec<ParameterMode> {
    let string = (opcode / 100).to_string();

    let chars = string.chars().rev().collect::<Vec<char>>();

    let mut res = Vec::new();

    for at_position in 1..=number_of_positions {
      if at_position > chars.len() {
        res.push(ParameterMode::Position);
        continue;
      }

      match chars[at_position - 1] {
        '0' => res.push(ParameterMode::Position),
        '1' => res.push(ParameterMode::Immediate),
        _ => continue,
      };
    }

    res
  }

  fn get(&self, cpu: &Intcode8086, at_position: usize) -> i32 {
    match self {
      ParameterMode::Immediate => cpu.von_neumann_tape[cpu.instruction_pointer + at_position],
      ParameterMode::Position => cpu.von_neumann_tape[cpu.von_neumann_tape[cpu.instruction_pointer + at_position] as usize]
    }
  }
}

struct InstructionArgument {
  mode: ParameterMode,
  value: i32
}

struct InstructionResult {
  next_instruction_pointer: Option<usize>,
  store: Option<StoreInstruction>,
}

struct StoreInstruction {
  address: usize,
  value: i32,
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
  fn test_parsing_instructions() {
    let cpu = Intcode8086::initialize(parse_csv("1,0,0,0,99"));
    let cpu = cpu.process().join().unwrap();
    assert_eq!(cpu.get_memory_at(0), 2);

    let cpu = Intcode8086::initialize(parse_csv("2,3,0,3,99"));
    let cpu = cpu.process().join().unwrap();
    assert_eq!(cpu.get_memory_at(0), 2);

    let cpu = Intcode8086::initialize(parse_csv("2,4,4,5,99,0"));
    let cpu = cpu.process().join().unwrap();
    assert_eq!(cpu.get_memory_at(0), 2);

    let cpu = Intcode8086::initialize(parse_csv("1,1,1,4,99,5,6,0,99"));
    let cpu = cpu.process().join().unwrap();
    assert_eq!(cpu.get_memory_at(0), 30);

    let cpu = Intcode8086::initialize(parse_csv("1002,4,3,4,33"));
    let cpu = cpu.process().join().unwrap();
    assert_eq!(cpu.get_memory_at(4), 99); // pos 4

    let cpu = Intcode8086::initialize(parse_csv("1101,100,-1,4,0"));
    let cpu = cpu.process().join().unwrap();
    assert_eq!(cpu.get_memory_at(4), 99); // pos 4
  }

  #[test]
  fn test_input_output() {
    let cpu = Intcode8086::initialize(parse_csv("3,0,4,0,99"));
    cpu.get_input_port().send(365).expect("Send should succeed");
    let io = cpu.get_output_port();

    let handle = cpu.process();

    handle.join().expect("");

    assert_eq!(io.recv().unwrap(), 365);
  }

  #[test]
  fn test_day5_part2_position_eq() {
    let cpu = Intcode8086::initialize(parse_csv("3,9,8,9,10,9,4,9,99,-1,8"));
    cpu.get_input_port().send(8).expect("Send should succeed");
    let io = cpu.get_output_port();

    let handle = cpu.process();

    handle.join().expect("");

    assert_eq!(io.recv().unwrap(), 1);
  }

  #[test]
  fn test_day5_part2_position_lt() {
    let mut cpu = Intcode8086::initialize(parse_csv("3,9,7,9,10,9,4,9,99,-1,8"));
    cpu.get_input_port().send(5).expect("Send should succeed");
    let io = cpu.get_output_port();

    let handle = cpu.process();

    handle.join().expect("");

    assert_eq!(io.recv().unwrap(), 1);
  }

  #[test]
  fn test_day5_part2_immediate_eq() {
    let mut cpu = Intcode8086::initialize(parse_csv("3,3,1108,-1,8,3,4,3,99"));
    cpu.get_input_port().send(8).expect("Send should succeed");
    let io = cpu.get_output_port();

    let handle = cpu.process();

    handle.join().expect("");

    assert_eq!(io.recv().unwrap(), 1);
  }

  #[test]
  fn test_day5_part2_immediate_lt() {
    let mut cpu = Intcode8086::initialize(parse_csv("3,3,1107,-1,8,3,4,3,99"));
    cpu.get_input_port().send(5).expect("Send should succeed");
    let io = cpu.get_output_port();

    let handle = cpu.process();

    handle.join().expect("");

    assert_eq!(io.recv().unwrap(), 1);
  }

  #[test]
  fn test_day5_part2_position_jump() {
    let mut cpu = Intcode8086::initialize(parse_csv("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9"));
    cpu.get_input_port().send(0).expect("Send should succeed");
    let io = cpu.get_output_port();

    let handle = cpu.process();

    handle.join().expect("");

    assert_eq!(io.recv().unwrap(), 0);
  }

  #[test]
  fn test_day5_part2_immediate_jump() {
    let mut cpu = Intcode8086::initialize(parse_csv("3,3,1105,-1,9,1101,0,0,12,4,12,99,1"));
    cpu.get_input_port().send(0).expect("Send should succeed");
    let io = cpu.get_output_port();

    let handle = cpu.process();

    handle.join().expect("");

    assert_eq!(io.recv().unwrap(), 0);
  }

  #[test]
  fn test_day5_part2_999_lt_8() {
    let mut cpu = Intcode8086::initialize(parse_csv("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99"));
    cpu.get_input_port().send(4).expect("Send should succeed");
    let io = cpu.get_output_port();

    let handle = cpu.process();

    handle.join().expect("");

    assert_eq!(io.recv().unwrap(), 999);
  }

  #[test]
  fn test_day5_part2_1000_eq_8() {
    let mut cpu = Intcode8086::initialize(parse_csv("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99"));
    cpu.get_input_port().send(8).expect("Send should succeed");
    let io = cpu.get_output_port();

    let handle = cpu.process();

    handle.join().expect("");

    assert_eq!(io.recv().unwrap(), 1000);
  }

  #[test]
  fn test_day5_part2_1001_gt_8() {
    let mut cpu = Intcode8086::initialize(parse_csv("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99"));
    cpu.get_input_port().send(9).expect("Send should succeed");
    let io = cpu.get_output_port();

    let handle = cpu.process();

    handle.join().expect("");

    assert_eq!(io.recv().unwrap(), 1001);
  }

  #[test]
  fn test_parsing_parameter_mode() {
    let pos = ParameterMode::parse(1002, 3);
    assert_eq!(ParameterMode::Position, pos[0]);
    assert_eq!(ParameterMode::Immediate, pos[1]);
    assert_eq!(ParameterMode::Position, pos[2]);
  }
}