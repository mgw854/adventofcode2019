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
      /*3 => Some(Instruction::StoreInput),
      4 => Some(Instruction::WriteOutput),
      5 => Some(Instruction::JumpIfTrue),
      6 => Some(Instruction::JumpIfFalse),
      7 => Some(Instruction::LessThan),
      8 => Some(Instruction::Equals),*/
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
}

enum Instruction {
  Add(ParameterMode, ParameterMode),
  Multiply(ParameterMode, ParameterMode),

  Halt
}

#[derive(PartialEq, Eq, Copy, Clone)]
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
  }