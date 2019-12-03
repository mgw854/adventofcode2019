use std::error::Error;

#[derive(Debug, PartialEq, Eq)]
pub struct WiringInstruction
{
  direction: Direction,
  run: u32
}

#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
  Left,
  Right,
  Up,
  Down
}

impl WiringInstruction {
  pub fn parse(input: &str) -> Result<WiringInstruction, Box<dyn Error>> {
    let dir = match input.chars().nth(0) {
      Some('L') => Direction::Left,
      Some('R') => Direction::Right,
      Some('U') => Direction::Up,
      Some('D') => Direction::Down,
      _ => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Bad direction")))
    };

    let run : u32 = input[1..].parse()?;

    Ok(WiringInstruction { direction : dir, run : run })
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_instruction() {
      assert_eq!(WiringInstruction::parse("D958").expect("Value wasn't parsable"), WiringInstruction { direction: Direction::Down, run: 958});
      assert_eq!(WiringInstruction::parse("U1").expect("Value wasn't parsable"), WiringInstruction { direction: Direction::Up, run: 1});
      assert_eq!(WiringInstruction::parse("L48").expect("Value wasn't parsable"), WiringInstruction { direction: Direction::Left, run: 48});
      assert_eq!(WiringInstruction::parse("R37").expect("Value wasn't parsable"), WiringInstruction { direction: Direction::Right, run: 37});
    }

}