const X_OFFSET: usize = 3;
const Y_OFFSET: usize = 1;

#[aoc(day3, part1)]
pub fn solve_part1(input: &str) -> i64 {
  let map: Vec<&str> = input.lines().collect();
  let mut count = 0;
  let height = map.len();
  let width = map[0].len();
  let mut x = 0;
  let mut y = 0;

  while y < height {
    if map[y].chars().nth(x).unwrap() == '#' {
      count += 1;
    }

    x = (x + X_OFFSET) % width;
    y += Y_OFFSET;
  }

  count
}

const OFFSETS: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

#[aoc(day3, part2)]
pub fn solve_part2(input: &str) -> i64 {
  let map: Vec<&str> = input.lines().collect();
  let height = map.len();
  let width = map[0].len();
  let mut count_product = 1;
  
  for offset in OFFSETS.iter() {
    let mut count = 0;
    let mut x = 0;
    let mut y = 0;

    while y < height {
      if map[y].chars().nth(x).unwrap() == '#' {
        count += 1;
      }

      x = (x + offset.0) % width;
      y += offset.1;
    }

    count_product *= count;
  }

  count_product
}

