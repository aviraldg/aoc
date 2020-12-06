use std::collections::HashSet;

const BAD_YEAR: i64 = 2020;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i64> {
  input.lines().map(|l| l.parse::<i64>().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[i64]) -> i64 {
  let entries: HashSet<i64> = input.iter().cloned().collect();
  for entry in &entries {
    if entries.contains(&(BAD_YEAR - entry)) {
      return entry * (BAD_YEAR - entry);
    }
  }
  panic!("invalid test case")
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[i64]) -> i64 {
  let mut entries: Vec<i64> = input.into();
  entries.sort();
  let entries_set: HashSet<i64> = input.iter().cloned().collect();
  for i in 0..entries.len() {
    for j in i + 1..entries.len() - 1 {
      let k_ = BAD_YEAR - entries[i] - entries[j];
      if entries_set.contains(&k_) {
        return entries[i] * entries[j] * k_;
      }
    }
  }
  panic!("invalid test case")
}
