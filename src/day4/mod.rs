pub fn adjacent_are_same(password: &str) -> bool {
  let vec = password.chars().collect::<Vec<char>>();
  let mut iter = vec.windows(2);

  for pair in iter {
    if pair[0] == pair[1] {
      return true;
    }
  }

  false
}

pub fn adjacent_never_decrease(password: &str) -> bool {
  let mut minVal = 0;

  for character in password.chars().map(|c| c.to_digit(10).unwrap()) {
    if minVal < character {
      minVal = character;
    } else if minVal > character {
      return false;
    }
  };

  true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adjacent_are_same() {
      assert_eq!(adjacent_are_same("122345"), true);
      assert_eq!(adjacent_are_same("123456"), false);
    }
 
    #[test]
    fn test_adjacent_never_decrease() {
      assert_eq!(adjacent_never_decrease("122345"), true);
      assert_eq!(adjacent_never_decrease("123456"), true);
      assert_eq!(adjacent_never_decrease("124368"), false);
    } }