pub struct Intcode {
  tape: Vec<usize>
}

pub fn create(tape: Vec<usize>) -> Intcode {
  Intcode { tape: tape }
}

impl Intcode {
  pub fn process(mut self) -> usize{
    let mut instruction_pointer = 0;
    
    while instruction_pointer < self.tape.len() {
      let res = match Instruction::parse(self.tape[instruction_pointer]) {
        Some(Instruction::Add) => self.three_arg_fn(instruction_pointer, |a, b| a + b),
        Some(Instruction::Multiply) => self.three_arg_fn(instruction_pointer, |a, b| a * b),
        Some(Instruction::Halt) => InstructionResult { next_instruction_pointer: None, store: None },
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
  
  fn three_arg_fn(&self, pointer: usize, func: fn(usize, usize) -> usize) -> InstructionResult
  {
    let store_address = self.tape[pointer + 3];
    let store_value = func(self.tape[self.tape[pointer + 1]], self.tape[self.tape[pointer + 2]]);

    InstructionResult {
      next_instruction_pointer: Some(pointer + 4),
      store: Some(StoreInstruction { address: store_address, value: store_value })
    }
  }
}

enum Instruction {
  Add,
  Multiply,
  Halt
}

impl Instruction {
  fn parse(opcode: usize) -> Option<Instruction> {
    match opcode {
      1 => Some(Instruction::Add),
      2 => Some(Instruction::Multiply),
      99 => Some(Instruction::Halt),
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
  value: usize
}


#[cfg(test)]
mod tests {
    use super::*;

    fn parse_csv(input: &str) -> Vec<usize> {
      input.split(",").map(|s| s.trim()).map(|s| s.parse::<usize>().unwrap()).collect()
    }

    #[test]
    fn test_parsing_instruction() {
      assert_eq!(create(parse_csv("1,0,0,0,99")).process(), 2); 
      assert_eq!(create(parse_csv("2,3,0,3,99")).process(), 2); 
      assert_eq!(create(parse_csv("2,4,4,5,99,0")).process(), 2); 
      assert_eq!(create(parse_csv("1,0,0,0,99")).process(), 2); 
      assert_eq!(create(parse_csv("1,1,1,4,99,5,6,0,99")).process(), 30); 
    }
}