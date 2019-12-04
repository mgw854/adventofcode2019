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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adjacent_are_same() {
      assert_eq!(adjacent_are_same("122345"), true);
      assert_eq!(adjacent_are_same("123456"), false);
    }
  }