use std::collections::HashSet;

#[aoc_generator(day6)]
fn input_generator(input: &str) -> Vec<HashSet<char>> {
  input
    .split("\n\n")
    .map(|group| {
      group
        .lines()
        .map(|person| person.chars().collect::<HashSet<char>>())
        .fold(('a'..='z').collect::<HashSet<char>>(), |i, e| {
          i.intersection(&e).copied().collect::<HashSet<char>>()
        })
    })
    .collect()
}

#[aoc(day6, part2)]
fn solve_part2(input: &Vec<HashSet<char>>) -> usize {
  input.iter().map(|set| set.len()).sum()
}
