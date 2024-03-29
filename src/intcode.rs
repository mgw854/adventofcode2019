use std::collections::VecDeque;

pub struct Intcode {
    tape: Vec<i32>,
    input: VecDeque<i32>,
    output: VecDeque<i32>,
    instruction_pointer: usize
}

pub enum IntcodeState {
  Halt { first_value: i32 },
  IOWait
}

impl Intcode {
    pub fn create(tape: Vec<i32>) -> Intcode {
        Intcode {
            tape: tape,
            input: VecDeque::new(),
            output: VecDeque::new(),
            instruction_pointer: 0
        }
    }

    pub fn push_input(&mut self, input: i32) {
      self.input.push_back(input);
    }

    pub fn process(&mut self) -> IntcodeState {
        while self.instruction_pointer < self.tape.len() {
            let res = match Instruction::parse(self.tape[self.instruction_pointer]) {
                Some(Instruction::Add) => self.three_arg_fn(self.instruction_pointer, |a, b| a + b),
                Some(Instruction::Multiply) => self.three_arg_fn(self.instruction_pointer, |a, b| a * b),
                Some(Instruction::StoreInput) => {
                  let res = self.store_input(self.instruction_pointer);

                  if res.is_none() {
                    return IntcodeState::IOWait;
                  }

                  res.unwrap()
                },
                Some(Instruction::WriteOutput) => self.write_output(self.instruction_pointer),

                Some(Instruction::JumpIfTrue) => self.jump(self.instruction_pointer, true),
                Some(Instruction::JumpIfFalse) => self.jump(self.instruction_pointer, false),
                Some(Instruction::LessThan) => self.compare_args(self.instruction_pointer, |a, b| a < b),
                Some(Instruction::Equals) => self.compare_args(self.instruction_pointer, |a, b| a == b),

                Some(Instruction::Halt) => InstructionResult {
                    next_instruction_pointer: None,
                    store: None,
                },
                None => panic!("Unknown instruction"),
            };

            match res.next_instruction_pointer {
                Some(x) => self.instruction_pointer = x,
                None => break,
            };

            if let Some(store) = res.store {
                self.tape[store.address] = store.value;
            }
        }

        let first_pos_value = self.tape[0];

        IntcodeState::Halt { first_value: first_pos_value }
    }

    pub fn read_output(&mut self) -> Option<i32> {
        self.output.pop_front()
    }

    fn jump(&self, pointer: usize, jump_if: bool) -> InstructionResult {
        let eval = self.get_parameter(pointer, 1);
        let next = match jump_if {
            true => {
                if eval != 0 {
                    self.get_parameter(pointer, 2) as usize
                } else {
                    pointer + 3
                }
            }
            false => {
                if eval == 0 {
                    self.get_parameter(pointer, 2) as usize
                } else {
                    pointer + 3
                }
            }
        };

        InstructionResult {
            next_instruction_pointer: Some(next),
            store: None,
        }
    }

    fn compare_args(&self, pointer: usize, func: fn(i32, i32) -> bool) -> InstructionResult {
        let store_address: usize = self.tape[pointer + 3] as usize;
        let result = func(
            self.get_parameter(pointer, 1),
            self.get_parameter(pointer, 2),
        );

        InstructionResult {
            next_instruction_pointer: Some(pointer + 4),
            store: Some(StoreInstruction {
                address: store_address,
                value: if result { 1 } else { 0 },
            }),
        }
    }

    fn store_input(&mut self, pointer: usize) -> Option<InstructionResult> {
      if let Some(input) = self.input.pop_front() {
          Some(InstructionResult {
            next_instruction_pointer: Some(pointer + 2),
            store: Some(StoreInstruction {
                address: self.tape[pointer + 1] as usize,
                value: input,
            })
          })
      } else {
        None
      }        
    }

    fn write_output(&mut self, pointer: usize) -> InstructionResult {
        self.output.push_back(self.get_parameter(pointer, 1));
        InstructionResult {
            next_instruction_pointer: Some(pointer + 2),
            store: None,
        }
    }

    fn three_arg_fn(&self, pointer: usize, func: fn(i32, i32) -> i32) -> InstructionResult {
        let store_address: usize = self.tape[pointer + 3] as usize;
        let store_value = func(
            self.get_parameter(pointer, 1),
            self.get_parameter(pointer, 2),
        );

        InstructionResult {
            next_instruction_pointer: Some(pointer + 4),
            store: Some(StoreInstruction {
                address: store_address,
                value: store_value,
            }),
        }
    }

    fn get_parameter(&self, instruction_pointer: usize, at_position: usize) -> i32 {
        match ParameterMode::parse(self.tape[instruction_pointer], at_position) {
            Some(ParameterMode::Immediate) => self.tape[instruction_pointer + at_position],
            Some(ParameterMode::Position) => {
                self.tape[self.tape[instruction_pointer + at_position] as usize]
            }
            None => panic!("Unknown parameter mode"),
        }
    }
}

enum Instruction {
    Add,
    Multiply,
    StoreInput,
    WriteOutput,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Halt,
}

impl Instruction {
    fn parse(opcode: i32) -> Option<Instruction> {
        match opcode % 100 {
            1 => Some(Instruction::Add),
            2 => Some(Instruction::Multiply),
            3 => Some(Instruction::StoreInput),
            4 => Some(Instruction::WriteOutput),
            5 => Some(Instruction::JumpIfTrue),
            6 => Some(Instruction::JumpIfFalse),
            7 => Some(Instruction::LessThan),
            8 => Some(Instruction::Equals),
            99 => Some(Instruction::Halt),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum ParameterMode {
    Position,
    Immediate,
}

impl ParameterMode {
    fn parse(opcode: i32, at_position: usize) -> Option<ParameterMode> {
        let string = (opcode / 100).to_string();

        let chars = string.chars().rev().collect::<Vec<char>>();

        if at_position > chars.len() {
            return Some(ParameterMode::Position);
        }

        match chars[at_position - 1] {
            '0' => Some(ParameterMode::Position),
            '1' => Some(ParameterMode::Immediate),
            _ => None,
        }
    }
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
/*
    #[test]
    fn test_parsing_instruction() {
        assert_eq!(Intcode::create(parse_csv("1,0,0,0,99")).process().1, 2);
        assert_eq!(Intcode::create(parse_csv("2,3,0,3,99")).process().1, 2);
        assert_eq!(Intcode::create(parse_csv("2,4,4,5,99,0")).process().1, 2);
        assert_eq!(Intcode::create(parse_csv("1,0,0,0,99")).process().1, 2);
        assert_eq!(
            Intcode::create(parse_csv("1,1,1,4,99,5,6,0,99"))
                .process()
                .1,
            30
        );

        assert_eq!(
            Intcode::create(parse_csv("1002,4,3,4,33")).process().0.tape[4],
            99
        );
        assert_eq!(
            Intcode::create(parse_csv("1101,100,-1,4,0"))
                .process()
                .0
                .tape[4],
            99
        );
    }*/

    #[test]
    fn test_input_output() {
        let mut cpu = Intcode::create(parse_csv("3,0,4,0,99"));
        cpu.push_input(365);
        let res = match cpu.process() {
          IntcodeState::IOWait => panic!("Not supposed to get here"),
          IntcodeState::Halt { first_value: _ } => cpu.read_output().unwrap()
        };

        assert_eq!(res, 365);    }

    #[test]
    fn test_day5_part2_position_eq() {
        let mut cpu = Intcode::create(parse_csv("3,9,8,9,10,9,4,9,99,-1,8"));
        cpu.push_input(8);
        let res = match cpu.process() {
          IntcodeState::IOWait => panic!("Not supposed to get here"),
          IntcodeState::Halt { first_value: _ } => cpu.read_output().unwrap()
        };

        assert_eq!(res, 1);    }

    #[test]
    fn test_day5_part2_position_lt() {
        let mut cpu = Intcode::create(parse_csv("3,9,7,9,10,9,4,9,99,-1,8"));
        cpu.push_input(5);
        let res = match cpu.process() {
          IntcodeState::IOWait => panic!("Not supposed to get here"),
          IntcodeState::Halt { first_value: _ } => cpu.read_output().unwrap()
        };

        assert_eq!(res, 1);    }

    #[test]
    fn test_day5_part2_immediate_eq() {
        let mut cpu = Intcode::create(parse_csv("3,3,1108,-1,8,3,4,3,99"));
        cpu.push_input(8);
        let res = match cpu.process() {
          IntcodeState::IOWait => panic!("Not supposed to get here"),
          IntcodeState::Halt { first_value: _ } => cpu.read_output().unwrap()
        };

        assert_eq!(res, 1);
    }

    #[test]
    fn test_day5_part2_immediate_lt() {
        let mut cpu = Intcode::create(parse_csv("3,3,1107,-1,8,3,4,3,99"));
        cpu.push_input(5);
        let res = match cpu.process() {
          IntcodeState::IOWait => panic!("Not supposed to get here"),
          IntcodeState::Halt { first_value: _ } => cpu.read_output().unwrap()
        };

        assert_eq!(res, 1);
    }

    #[test]
    fn test_day5_part2_position_jump() {
        let mut cpu = Intcode::create(parse_csv("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9"));
        cpu.push_input(0);
        let res = match cpu.process() {
          IntcodeState::IOWait => panic!("Not supposed to get here"),
          IntcodeState::Halt { first_value: _ } => cpu.read_output().unwrap()
        };

        assert_eq!(res, 0);
    }

    #[test]
    fn test_day5_part2_immediate_jump() {
        let mut cpu = Intcode::create(parse_csv("3,3,1105,-1,9,1101,0,0,12,4,12,99,1"));
        cpu.push_input(0);
        let res = match cpu.process() {
          IntcodeState::IOWait => panic!("Not supposed to get here"),
          IntcodeState::Halt { first_value: _ } => cpu.read_output().unwrap()
        };

        assert_eq!(res, 0);
    }

    #[test]
    fn test_day5_part2_999_lt_8() {
        let mut cpu = Intcode::create(parse_csv("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99"));
        cpu.push_input(4);
        let res = match cpu.process() {
          IntcodeState::IOWait => panic!("Not supposed to get here"),
          IntcodeState::Halt { first_value: _ } => cpu.read_output().unwrap()
        };

        assert_eq!(res, 999);
    }

    #[test]
    fn test_day5_part2_1000_eq_8() {
        let mut cpu = Intcode::create(parse_csv("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99"));
        cpu.push_input(8);
        let res = match cpu.process() {
          IntcodeState::IOWait => panic!("Not supposed to get here"),
          IntcodeState::Halt { first_value: _ } => cpu.read_output().unwrap()
        };

        assert_eq!(res, 1000);
    }

    #[test]
    fn test_day5_part2_1001_gt_8() {
        let mut cpu = Intcode::create(parse_csv("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99"));
        cpu.push_input(9);

        let res = match cpu.process() {
          IntcodeState::IOWait => panic!("Not supposed to get here"),
          IntcodeState::Halt { first_value: _ } => cpu.read_output().unwrap()
        };

        assert_eq!(res, 1001);
    }

    #[test]
    fn test_parsing_parameter_mode() {
        assert_eq!(ParameterMode::parse(1002, 1), Some(ParameterMode::Position));
        assert_eq!(
            ParameterMode::parse(1002, 2),
            Some(ParameterMode::Immediate)
        );
        assert_eq!(ParameterMode::parse(1002, 3), Some(ParameterMode::Position));
    }
}
