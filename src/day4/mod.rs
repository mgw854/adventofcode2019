pub fn adjacent_are_same(password: &str) -> bool {
  let mut vec = password.chars().collect::<Vec<char>>();

  let mut ignore = Vec::<usize>::new();

  let mut prevChar = ' ';
  let mut hits = 0;
  let mut index = 0;

  for character in vec.iter() {
    if *character == prevChar {
      hits += 1;
    }
    else {
      if hits > 2 {
        while hits > 0 {
          ignore.push(index - hits);
          hits -= 1;
        }
      }

      prevChar = *character;
      hits = 1;
    }

    index += 1;
  }

  if hits > 2 {
    while hits > 0 {
      ignore.push(index - hits);
      hits -= 1;
    }
  }

  ignore.sort();

  for i in ignore.iter().rev() {
    vec.remove(*i);
  }

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
      assert_eq!(adjacent_are_same("112233"), true);
      assert_eq!(adjacent_are_same("111122"), true);
      assert_eq!(adjacent_are_same("123444"), false);
    }
 
    #[test]
    fn test_adjacent_never_decrease() {
      assert_eq!(adjacent_never_decrease("122345"), true);
      assert_eq!(adjacent_never_decrease("123456"), true);
      assert_eq!(adjacent_never_decrease("124368"), false);
    } }