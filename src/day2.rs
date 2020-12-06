use std::str::FromStr;
use text_io::scan;

#[derive(Debug, Default)]
pub struct PasswordPolicy {
  password: String,
  character: char,
  min: u64,
  max: u64,
}

#[derive(Debug)]
pub struct PasswordPolicyParseErr();

impl FromStr for PasswordPolicy {
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut password_policy = PasswordPolicy::default();
    scan!(s.bytes() => "{}-{} {}: {}", password_policy.min, password_policy.max, password_policy.character, password_policy.password);
    Ok(password_policy)
  }

  type Err = PasswordPolicyParseErr;
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<PasswordPolicy> {
  input
    .lines()
    .map(|l| l.parse::<PasswordPolicy>().unwrap())
    .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(inputs: &[PasswordPolicy]) -> i64 {
  let mut valid_count = 0;
  for input in inputs {
    let mut count = 0;
    for char in input.password.chars() {
      if char == input.character {
        count+=1;
      }
    }
    if input.min <= count && count <= input.max {
      valid_count += 1
    }
  }
  valid_count
}

#[aoc(day2, part2)]
pub fn solve_part2(inputs: &[PasswordPolicy]) -> i64 {
  let mut valid_count = 0;
  for input in inputs {
    if (input.password.chars().nth((input.min-1) as usize).unwrap() == input.character) ^ (input.password.chars().nth((input.max-1) as usize).unwrap() == input.character) {
      valid_count += 1;
    }
  }
  valid_count
}
