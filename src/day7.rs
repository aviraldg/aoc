use std::{collections::{HashMap, HashSet, VecDeque}};

#[aoc_generator(day7)]
fn input_generator(input: &str) -> HashMap<String, Vec<(String, i64)>> {
  input
    .lines()
    .map(|line| -> (String, Vec<(String, i64)>) {
      let parts: Vec<&str> = line.split(" contain ").collect();
      let map: Vec<(String, i64)> = parts[1]
        .strip_suffix(".")
        .unwrap()
        .split(", ")
        .filter_map(|part| {
          if part == "no other bags" {
            return None;
          }
          let sections: Vec<&str> = part.split(" ").collect();
          let name = sections[1..].join(" ");
          let name_singular = name.strip_suffix("s").unwrap_or(&name);
          Some((
            name_singular.to_string(),
            sections[0].parse::<i64>().unwrap(),
          ))
        })
        .collect();
      (parts[0].strip_suffix("s").unwrap().to_string(), map)
    })
    .collect()
}

fn flip_map(input: &HashMap<String, Vec<(String, i64)>>) -> HashMap<String, HashSet<String>> {
  let mut result: HashMap<String, HashSet<String>> = HashMap::new();

  for (k, vmap) in input {
    for (v, _) in vmap {
      result
        .entry(v.to_string())
        .or_default()
        .insert(k.to_string());
    }
  }

  result
}

#[aoc(day7, part1)]
fn solve_part1(input: &HashMap<String, Vec<(String, i64)>>) -> usize {
  let map = flip_map(input);

  let mut cache: HashSet<String> = HashSet::new();
  let mut q: VecDeque<&str> = VecDeque::new();
  q.push_back("shiny gold bag");
  while let Some(top) = q.pop_front() {
    if cache.contains(top) {
      continue;
    }
    if let Some(children) = map.get(top) {
      children.iter().for_each(|e| {
        q.push_back(e);
      });
    }
    cache.insert(top.to_string());
  }
  cache.len() - 1
}

// TODO(aviraldg): Fix this. It's a *horrible* hack. Should've used structs from the start.
#[aoc(day7, part2)]
fn solve_part2(input: &HashMap<String, Vec<(String, i64)>>) -> i64 {
  let mut stk = Vec::new();
  let mut sum = Vec::new();
  let start = ("shiny gold bag".to_string(), 0usize);
  let mut root_or = Some(start.clone());
  while root_or.is_some() {
    while let Some(ref root) = root_or {
      stk.push(root.clone());
      sum.push(0);

      match input.get(&root.0) {
        Some(children) => match children.first() {
          Some(first_child) => {
            root_or = Some((first_child.0.clone(), 0));
          }
          None => break,
        },
        None => break,
      }
    }

    while stk.len() >= 2 {
      let top = stk.pop().unwrap();
      let my_sum = sum.pop().unwrap();
      let count = input[&stk.last().unwrap().0][top.1].1;
      *sum.last_mut().unwrap() = sum.last().unwrap() + (1+my_sum)*count;
      
      if input[&stk.last().unwrap().0].len() > top.1 + 1 {
        root_or = Some((input[&stk.last().unwrap().0][top.1 + 1].0.clone(), top.1+1));
        break;
      } else {
        root_or = Some(stk.last().unwrap().clone());
      }
    }
    if stk.len() == 1 && root_or == Some(start.clone()) {
      break;
    }
  }
  sum[0]
}
