use std::thread;
use crossbeam_channel::{ Sender, Receiver, unbounded };
use bus::Bus;

pub struct Intcode8086 {
  instruction_pointer: usize,
  relative_base_pointer: usize,
  von_neumann_tape: Vec<i64>,
  input_sender: Sender<i64>,
  input_receiver: Receiver<i64>,
  output_bus: Bus<i64>
}

impl Intcode8086 {
  pub fn initialize(von_neumann_tape: Vec<i64>) -> Intcode8086 {
    let (i_s, i_r) = unbounded();

    Intcode8086 {
      instruction_pointer: 0,
      relative_base_pointer: 0,
      von_neumann_tape: von_neumann_tape,
      input_sender: i_s,
      input_receiver: i_r,
      output_bus: Bus::new(100)
    }
  }

  pub fn get_input_port(&self) -> Sender<i64> {
    self.input_sender.clone()
  }

  pub fn get_output_port(&mut self) -> bus::BusReader<i64> {
    self.output_bus.add_rx()
  }

  pub fn process(mut self) -> std::thread::JoinHandle<Self> {
    thread::spawn(move || {
      while self.instruction_pointer < self.von_neumann_tape.len() {
        let instruction = Intcode8086::decode_instruction(self.von_neumann_tape[self.instruction_pointer] as usize);

        let res = match instruction {
          Some(Instruction::Add(arg1, arg2, arg3)) => self.three_arg_fn(arg1, arg2, |a, b| a + b, arg3),
          Some(Instruction::Multiply(arg1, arg2, arg3)) => self.three_arg_fn(arg1, arg2, |a, b| a * b, arg3),
          Some(Instruction::StoreInput(arg1)) => self.store_input(arg1),
          Some(Instruction::WriteOutput(arg1)) => self.write_output(arg1),
          Some(Instruction::JumpIfTrue(arg1, arg2)) => self.jump(arg1, arg2, true),
          Some(Instruction::JumpIfFalse(arg1, arg2)) => self.jump(arg1, arg2, false),
          Some(Instruction::LessThan(arg1, arg2, arg3)) => self.compare_args(arg1, arg2, |a, b| a < b, arg3),
          Some(Instruction::Equals(arg1, arg2, arg3)) => self.compare_args(arg1, arg2, |a, b| a == b, arg3),
          Some(Instruction::AdjustRelativeBase(arg1)) => self.adjust_relative_base(arg1),
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
          if store.address >= self.von_neumann_tape.len() {
            for i in self.von_neumann_tape.len()..=store.address {
              self.von_neumann_tape.push(0);
            }
          }

          self.von_neumann_tape[store.address] = store.value;
        }
      }

      self
    })
  }

  pub fn get_memory_at(&self, position: usize) -> i64 {
    self.von_neumann_tape[position]
  }

  fn decode_instruction(opcode: usize) -> Option<Instruction> {
    match opcode % 100 {
      1 => { let p = ParameterMode::parse(opcode, 3); Some(Instruction::Add(p[0], p[1], p[2])) }
      2 => { let p = ParameterMode::parse(opcode, 3); Some(Instruction::Multiply(p[0], p[1], p[2])) }
      3 => { let p = ParameterMode::parse(opcode, 1); Some(Instruction::StoreInput(p[0])) },
      4 => { let p = ParameterMode::parse(opcode, 1); Some(Instruction::WriteOutput(p[0])) },
      5 => { let p = ParameterMode::parse(opcode, 2); Some(Instruction::JumpIfTrue(p[0], p[1])) },
      6 => { let p = ParameterMode::parse(opcode, 2); Some(Instruction::JumpIfFalse(p[0], p[1])) },
      7 => { let p = ParameterMode::parse(opcode, 3); Some(Instruction::LessThan(p[0], p[1], p[2])) },
      8 => { let p = ParameterMode::parse(opcode, 3); Some(Instruction::Equals(p[0], p[1], p[2])) },
      9 => { let p = ParameterMode::parse(opcode, 1); Some(Instruction::AdjustRelativeBase(p[0])) }
      99 => Some(Instruction::Halt),
      _ => None,
    }
  }

  fn three_arg_fn(&self, arg1: ParameterMode, arg2: ParameterMode, func: fn(i64, i64) -> i64, arg3: ParameterMode) -> InstructionResult {
    let store_address: usize = arg3.set(self, 3);
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

  fn compare_args(&self, arg1: ParameterMode, arg2: ParameterMode, func: fn(i64, i64) -> bool, arg3: ParameterMode) -> InstructionResult {
    let store_address: usize = arg3.set(self, 3);
    let result = func(arg1.get(self, 1), arg2.get(self, 2));

    InstructionResult {
      next_instruction_pointer: Some(self.instruction_pointer + 4),
      store: Some(StoreInstruction {
          address: store_address,
          value: if result { 1 } else { 0 },
      }),
    }
  }

  fn store_input(&mut self, arg1: ParameterMode) -> InstructionResult {
    let address = arg1.set(self, 1);
    
    if address >= self.von_neumann_tape.len() {
      for i in self.von_neumann_tape.len()..=address {
        self.von_neumann_tape.push(0);
      }
    }

    InstructionResult {
      next_instruction_pointer: Some(self.instruction_pointer + 2),
      store: Some(StoreInstruction {
        address: address,
        value: self.input_receiver.recv().unwrap()
      })
    }    
  }

  fn write_output(&mut self, arg1: ParameterMode) -> InstructionResult {
    self.output_bus.broadcast(arg1.get(self, 1));
    InstructionResult {
      next_instruction_pointer: Some(self.instruction_pointer + 2),
      store: None,
    }
  }

  fn adjust_relative_base(&mut self, arg1: ParameterMode) -> InstructionResult {
    self.relative_base_pointer = (self.relative_base_pointer as i64 + arg1.get(self, 1)) as usize;

    InstructionResult {
      next_instruction_pointer: Some(self.instruction_pointer + 2),
      store: None,
    }
  }
}

enum Instruction {
  Add(ParameterMode, ParameterMode, ParameterMode),
  Multiply(ParameterMode, ParameterMode, ParameterMode),
  StoreInput(ParameterMode),
  WriteOutput(ParameterMode),
  JumpIfTrue(ParameterMode, ParameterMode),
  JumpIfFalse(ParameterMode, ParameterMode),
  LessThan(ParameterMode, ParameterMode, ParameterMode),
  Equals(ParameterMode, ParameterMode, ParameterMode),
  AdjustRelativeBase(ParameterMode),

  Halt
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum ParameterMode {
  Position,
  Immediate,
  Relative
}

impl std::fmt::Display for ParameterMode {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      ParameterMode::Position => write!(f, "Position"),
      ParameterMode::Immediate => write!(f, "Immediate"),
      ParameterMode::Relative => write!(f, "Relative")
    }
  }
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
        '2' => res.push(ParameterMode::Relative),
        _ => continue
      };
    }

    res
  }

  fn get(&self, cpu: &Intcode8086, at_position: usize) -> i64 {
    let addr = match self {
      ParameterMode::Immediate => cpu.instruction_pointer + at_position,
      ParameterMode::Position => cpu.von_neumann_tape[cpu.instruction_pointer + at_position] as usize,
      ParameterMode::Relative => (cpu.von_neumann_tape[cpu.instruction_pointer + at_position] + cpu.relative_base_pointer as i64) as usize
    };

    if addr >= cpu.von_neumann_tape.len() {
      0      
    } else {
      cpu.von_neumann_tape[addr]
    }
  }

  fn set(&self, cpu: &Intcode8086, at_position: usize) -> usize {
    match self {
      ParameterMode::Relative => (cpu.relative_base_pointer as i64 + cpu.von_neumann_tape[cpu.instruction_pointer + at_position]) as usize,
      _ => cpu.von_neumann_tape[cpu.instruction_pointer + at_position] as usize
    }
  }
}

struct InstructionArgument {
  mode: ParameterMode,
  value: i64
}

struct InstructionResult {
  next_instruction_pointer: Option<usize>,
  store: Option<StoreInstruction>,
}

struct StoreInstruction {
  address: usize,
  value: i64,
}

#[cfg(test)]
mod tests {
  use super::*;

  fn parse_csv(input: &str) -> Vec<i64> {
      input
          .split(",")
          .map(|s| s.trim())
          .map(|s| s.parse::<i64>().unwrap())
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
    let mut cpu = Intcode8086::initialize(parse_csv("3,0,4,0,99"));
    cpu.get_input_port().send(365).expect("Send should succeed");
    let mut io = cpu.get_output_port();

    let handle = cpu.process();

    handle.join().expect("");

    assert_eq!(io.recv().unwrap(), 365);
  }

  #[test]
  fn test_day5_part2_position_eq() {
    let mut cpu = Intcode8086::initialize(parse_csv("3,9,8,9,10,9,4,9,99,-1,8"));
    cpu.get_input_port().send(8).expect("Send should succeed");
    let mut io = cpu.get_output_port();

    let handle = cpu.process();

    handle.join().expect("");

    assert_eq!(io.recv().unwrap(), 1);
  }

  #[test]
  fn test_day5_part2_position_lt() {
    let mut cpu = Intcode8086::initialize(parse_csv("3,9,7,9,10,9,4,9,99,-1,8"));
    cpu.get_input_port().send(5).expect("Send should succeed");
    let mut io = cpu.get_output_port();

    let handle = cpu.process();

    handle.join().expect("");

    assert_eq!(io.recv().unwrap(), 1);
  }

  #[test]
  fn test_day5_part2_immediate_eq() {
    let mut cpu = Intcode8086::initialize(parse_csv("3,3,1108,-1,8,3,4,3,99"));
    cpu.get_input_port().send(8).expect("Send should succeed");
    let mut io = cpu.get_output_port();

    let handle = cpu.process();

    handle.join().expect("");

    assert_eq!(io.recv().unwrap(), 1);
  }

  #[test]
  fn test_day5_part2_immediate_lt() {
    let mut cpu = Intcode8086::initialize(parse_csv("3,3,1107,-1,8,3,4,3,99"));
    cpu.get_input_port().send(5).expect("Send should succeed");
    let mut io = cpu.get_output_port();

    let handle = cpu.process();

    handle.join().expect("");

    assert_eq!(io.recv().unwrap(), 1);
  }

  #[test]
  fn test_day5_part2_position_jump() {
    let mut cpu = Intcode8086::initialize(parse_csv("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9"));
    cpu.get_input_port().send(0).expect("Send should succeed");
    let mut io = cpu.get_output_port();

    let handle = cpu.process();

    handle.join().expect("");

    assert_eq!(io.recv().unwrap(), 0);
  }

  #[test]
  fn test_day5_part2_immediate_jump() {
    let mut cpu = Intcode8086::initialize(parse_csv("3,3,1105,-1,9,1101,0,0,12,4,12,99,1"));
    cpu.get_input_port().send(0).expect("Send should succeed");
    let mut io = cpu.get_output_port();

    let handle = cpu.process();

    handle.join().expect("");

    assert_eq!(io.recv().unwrap(), 0);
  }

  #[test]
  fn test_day5_part2_999_lt_8() {
    let mut cpu = Intcode8086::initialize(parse_csv("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99"));
    cpu.get_input_port().send(4).expect("Send should succeed");
    let mut io = cpu.get_output_port();

    let handle = cpu.process();

    handle.join().expect("");

    assert_eq!(io.recv().unwrap(), 999);
  }

  #[test]
  fn test_day5_part2_1000_eq_8() {
    let mut cpu = Intcode8086::initialize(parse_csv("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99"));
    cpu.get_input_port().send(8).expect("Send should succeed");
    let mut io = cpu.get_output_port();

    let handle = cpu.process();

    handle.join().expect("");

    assert_eq!(io.recv().unwrap(), 1000);
  }

  #[test]
  fn test_day5_part2_1001_gt_8() {
    let mut cpu = Intcode8086::initialize(parse_csv("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99"));
    cpu.get_input_port().send(9).expect("Send should succeed");
    let mut io = cpu.get_output_port();

    let handle = cpu.process();

    handle.join().expect("");

    assert_eq!(io.recv().unwrap(), 1001);
  }

  #[test]
  fn test_day9_relative_base() {
    let mut cpu = Intcode8086::initialize(parse_csv("109,2000,109,19,99"));
    cpu = cpu.process().join().expect("");
    assert_eq!(cpu.relative_base_pointer, 2019);
  }

  #[test]
  fn test_day9_part1_copy() {
    let mut cpu = Intcode8086::initialize(parse_csv("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99"));
    let mut io = cpu.get_output_port();

    let handle = cpu.process();

    handle.join().expect("");

    assert_eq!(io.recv().unwrap(), 109);
    assert_eq!(io.recv().unwrap(), 1);
    assert_eq!(io.recv().unwrap(), 204);
    assert_eq!(io.recv().unwrap(), -1);
    assert_eq!(io.recv().unwrap(), 1001);
    assert_eq!(io.recv().unwrap(), 100);
    assert_eq!(io.recv().unwrap(), 1);
    assert_eq!(io.recv().unwrap(), 100);
    assert_eq!(io.recv().unwrap(), 1008);
    assert_eq!(io.recv().unwrap(), 100);
    assert_eq!(io.recv().unwrap(), 16);
    assert_eq!(io.recv().unwrap(), 101);
    assert_eq!(io.recv().unwrap(), 1006);
    assert_eq!(io.recv().unwrap(), 101);
    assert_eq!(io.recv().unwrap(), 0);
    assert_eq!(io.recv().unwrap(), 99);
  }

  #[test]
  fn test_day9_part1_output16digits() {
    let mut cpu = Intcode8086::initialize(parse_csv("1102,34915192,34915192,7,4,7,99,0"));
    let mut io = cpu.get_output_port();

    let handle = cpu.process();

    handle.join().expect("");

    let digits = io.recv().unwrap().to_string().len();

    assert_eq!(digits, 16);
  }

  #[test]
  fn test_day9_part1_output1125899906842624() {
    let mut cpu = Intcode8086::initialize(parse_csv("104,1125899906842624,99"));
    let mut io = cpu.get_output_port();

    let handle = cpu.process();

    handle.join().expect("");

    assert_eq!(io.recv().unwrap(), 1125899906842624);
  }

  #[test]
  fn test_parsing_parameter_mode() {
    let pos = ParameterMode::parse(1002, 3);
    assert_eq!(ParameterMode::Position, pos[0]);
    assert_eq!(ParameterMode::Immediate, pos[1]);
    assert_eq!(ParameterMode::Position, pos[2]);
  }
}