use std::collections::{HashMap, HashSet};

const SECTIONS: [&str; 8] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];
const NEEDED_SECTIONS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

#[aoc_generator(day4)]
fn input_generator(input: &str) -> Vec<HashMap<String, String>> {
  input.split("\n\n").map(|l| {
    l.split_whitespace().map(|p| {
      let vec: Vec<&str> = p.split(":").collect();
      (vec[0].to_string(), vec[1].to_string())
    }).collect()
  }).collect()
}

#[aoc(day4, part1)]
fn solve_part1(passports: &Vec<HashMap<String, String>>) -> usize {
  passports.iter().filter(|passport| {
    passport.keys().all(|key| SECTIONS.contains(&key.as_str())) && NEEDED_SECTIONS.iter().all(|key| passport.contains_key(*key))
  }).count()
}

#[aoc(day4, part2)]
fn solve_part2(passports: &Vec<HashMap<String, String>>) -> usize {
  let ecl_valid_values: HashSet<&str> = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
    .iter()
    .cloned()
    .collect();

  passports
    .iter()
    .filter(|passport| {
      NEEDED_SECTIONS.iter().all(|section| {
        if let Some(value) = passport.get(*section) {
          match *section {
            "byr" => (1920..=2002).contains(&value.parse::<i64>().unwrap()),
            "iyr" => (2010..=2020).contains(&value.parse::<i64>().unwrap()),
            "eyr" => (2020..=2030).contains(&value.parse::<i64>().unwrap()),
            "hgt" => match value.split_at(value.len() - 2) {
              (cmv, "cm") => (150..=193).contains(&cmv.parse::<i64>().unwrap()),
              (inv, "in") => (59..=76).contains(&inv.parse::<i64>().unwrap()),
              _ => false,
            },
            "hcl" => {
              value.len() == 7
                && value
                  .chars()
                  .skip(1)
                  .all(|c| ('0'..='9').contains(&c) || ('a'..='f').contains(&c))
            }
            "ecl" => ecl_valid_values.contains(value.as_str()),
            "pid" => value.len() == 9 && value.chars().all(|c| ('0'..='9').contains(&c)),
            _ => panic!(),
          }
        } else {
          false
        }
      })
    })
    .count()
}
