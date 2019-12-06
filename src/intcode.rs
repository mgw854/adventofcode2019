pub struct Intcode {
  tape: Vec<i32>,
  pub input: i32,
  output: Option<i32>
}

impl Intcode {
  pub fn create(tape: Vec<i32>) -> Intcode {
    Intcode { tape: tape, input: 0, output: None }
  }

  pub fn process(mut self) -> i32 {
    let mut instruction_pointer = 0;
    
    while instruction_pointer < self.tape.len() {
      let res = match Instruction::parse(self.tape[instruction_pointer]) {
        Some(Instruction::Add) => self.three_arg_fn(instruction_pointer, |a, b| a + b),
        Some(Instruction::Multiply) => self.three_arg_fn(instruction_pointer, |a, b| a * b),
        Some(Instruction::Halt) => InstructionResult { next_instruction_pointer: None, store: None },
        Some(x) => panic!("Unknown instruction"),
        None => panic!("Unknown instruction")
      };

      match res.next_instruction_pointer {
        Some(x) => instruction_pointer = x,
        None => break
      };

      if let Some(store) = res.store {
        self.tape[store.address] = store.value;
      }
    }

    self.tape[0]
  }

  pub fn read_output(&self) -> Option<i32> {
    self.output
  }

  fn three_arg_fn(&self, pointer: usize, func: fn(i32, i32) -> i32) -> InstructionResult
  {
    let store_address : usize = self.tape[pointer + 3] as usize;
    let store_value = func(self.tape[self.tape[pointer + 1] as  usize], self.tape[self.tape[pointer + 2] as usize]);

    InstructionResult {
      next_instruction_pointer: Some(pointer + 4),
      store: Some(StoreInstruction { address: store_address, value: store_value })
    }
  }
}

enum Instruction {
  Add,
  Multiply,
  StoreInput,
  WriteOutput,
  Halt
}

impl Instruction {
  fn parse(opcode: i32) -> Option<Instruction> {
    match opcode % 100 {
      1 => Some(Instruction::Add),
      2 => Some(Instruction::Multiply),
      3 => Some(Instruction::StoreInput),
      4 => Some(Instruction::WriteOutput),
      99 => Some(Instruction::Halt),
      _ => None
    }
  }
}

#[derive(Debug, PartialEq, Eq)]
enum ParameterMode {
  Position,
  Immediate
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
      _ => None
    }
  }
}

struct InstructionResult
{
  next_instruction_pointer: Option<usize>,
  store: Option<StoreInstruction>
}

struct StoreInstruction {
  address: usize,
  value: i32
}


#[cfg(test)]
mod tests {
    use super::*;

    fn parse_csv(input: &str) -> Vec<i32> {
      input.split(",").map(|s| s.trim()).map(|s| s.parse::<i32>().unwrap()).collect()
    }

    #[test]
    fn test_parsing_instruction() {
      assert_eq!(Intcode::create(parse_csv("1,0,0,0,99")).process(), 2); 
      assert_eq!(Intcode::create(parse_csv("2,3,0,3,99")).process(), 2); 
      assert_eq!(Intcode::create(parse_csv("2,4,4,5,99,0")).process(), 2); 
      assert_eq!(Intcode::create(parse_csv("1,0,0,0,99")).process(), 2); 
      assert_eq!(Intcode::create(parse_csv("1,1,1,4,99,5,6,0,99")).process(), 30); 
    }

    #[test]
    fn test_parsing_parameter_mode(){
      assert_eq!(ParameterMode::parse(1002, 1), Some(ParameterMode::Position));
      assert_eq!(ParameterMode::parse(1002, 2), Some(ParameterMode::Immediate));
      assert_eq!(ParameterMode::parse(1002, 3), Some(ParameterMode::Position));
    }
}