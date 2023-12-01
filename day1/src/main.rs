use std::collections::HashMap;
use std::fs;


fn extract_word<'a>(s: &'a str, digit_map: &HashMap<&str, i32>) -> Option<&'a str> {
  for start in 0..s.len() {
    for end in start + 1..=s.len() {
      let substr = &s[start..end];
      if digit_map.contains_key(substr) {
        return Some(substr);
      }
    }
  }
  None
}

fn part_one() -> usize {
  let input = fs::read_to_string("./src/input.txt").expect("Failed to read file");

  let mut result: usize = 0;

  for line in input.lines() {
    let first_digit = line.chars().find(|c| c.is_ascii_digit()).unwrap_or('0');

    let last_digit = line
      .chars()
      .rev()
      .find(|c| c.is_ascii_digit())
      .unwrap_or('0');

    let number = format!("{}{}", first_digit, last_digit);

    match number.parse::<usize>() {
      Ok(n) => result += n,
      Err(e) => eprintln!("Failed to parse number: {}", e),
    }
  }

  result
}

fn part_two() -> usize {
  let input = fs::read_to_string("./src/input.txt").expect("Failed to read file");

  let digit_map: HashMap<&str, i32> = HashMap::from([
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
  ]);

  let mut result: usize = 0;

  for line in input.lines() {
    let mut number_string = String::new();
    let mut probably_digit = String::new();

    for ch in line.chars() {
      if ch.is_ascii_digit() {
        probably_digit.clear();
        number_string.push(ch);

        break;
      } else {
        probably_digit.push(ch);

        if let Some(digit_word) = extract_word(probably_digit.as_str(), &digit_map) {
          let &value = digit_map.get(digit_word).unwrap();

          probably_digit.clear();
          number_string.push_str(value.to_string().as_str());

          break;
        }
      }
    }

    for ch in line.chars().rev() {
      if ch.is_ascii_digit() {
        probably_digit.clear();
        number_string.push(ch);

        break;
      } else {
        probably_digit.insert(0, ch);

        if let Some(digit_word) = extract_word(probably_digit.as_str(), &digit_map) {
          let &value = digit_map.get(digit_word).unwrap();

          probably_digit.clear();
          number_string.push_str(value.to_string().as_str());

          break;
        }
      }
    }

    result += number_string
      .parse::<usize>()
      .expect("Failed to parse number string");
  }

  result
}

fn main() {
  let part_one_result = part_one();
  println!("Part 1 result = {}", part_one_result);

  let part_two_result = part_two();
  println!("Part 2 result = {}", part_two_result);
}
