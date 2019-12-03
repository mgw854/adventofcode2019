use std::error::Error;
use std::collections::HashSet;

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

#[derive(Eq, PartialEq, Debug, Hash)]
pub struct Point {
  x: i32,
  y: i32
}

pub fn generate_points(instructions: &Vec<WiringInstruction>) -> HashSet<Point> {
  let mut set : HashSet<Point> = HashSet::new();

  let mut originX = 0;
  let mut originY = 0;
  
  for instr in instructions {
    match instr.direction {
      Direction::Left => {
        for len in 0..instr.run
        {
          originX -= 1;
          set.insert(Point { x: originX, y: originY });
        }
      },
      Direction::Right => {
        for len in 0..instr.run
        {
          originX += 1;
          set.insert(Point { x: originX, y: originY });
        }
      },
      Direction::Up => {
        for len in 0..instr.run
        {
          originY += 1;
          set.insert(Point { x: originX, y: originY });
        }
      },
      Direction::Down => {
        for len in 0..instr.run
        {
          originY -= 1;
          set.insert(Point { x: originX, y: originY });
        }
      }
    }
  };

  set.remove(&Point { x: 0, y: 0 });

  set
}

pub fn generate_manhattan_distance(one: &HashSet<Point>, two: &HashSet<Point>) -> u32 {
  one.intersection(two).map(|pt| i32::abs(pt.x) + i32::abs(pt.y)).min().unwrap() as u32
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

    #[test]
    fn test_manhattan_distance() {
      let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72
      U62,R66,U55,R34,D71,R55,D58,R83";

      let directions : Vec<Vec<WiringInstruction>> = input
      .lines()
      .map(|s| s.trim().split(",").map(|x| WiringInstruction::parse(x).unwrap()).collect())
      .collect();
      
      let one = generate_points(&directions[0]);
      let two = generate_points(&directions[1]);

      let distance = generate_manhattan_distance(&one, &two);

      assert_eq!(distance, 159);
    }

    #[test]
    fn test_manhattan_distance2() {
      let input = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
      U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";

      let directions : Vec<Vec<WiringInstruction>> = input
      .lines()
      .map(|s| s.trim().split(",").map(|x| WiringInstruction::parse(x).unwrap()).collect())
      .collect();
      
      let one = generate_points(&directions[0]);
      let two = generate_points(&directions[1]);

      let distance = generate_manhattan_distance(&one, &two);

      assert_eq!(distance, 135);
    }}