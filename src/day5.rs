fn solve(input: &str) -> impl Iterator<Item = u64> + '_ {
  input.lines().map(|bsp| {
    let mut f = 0;
    let mut b = 127;
    let mut l = 0;
    let mut r = 7;

    for c in bsp.chars() {
      match c {
        'F' => b = (f + b) / 2,
        'B' => {
          f = (f + b) / 2 + 1;
        }
        'L' => {
          r = (l + r) / 2;
        }
        'R' => {
          l = (l + r) / 2 + 1;
        }
        _ => panic!(),
      }
    }

    assert_eq!(f, b);
    assert_eq!(l, r);

    f * 8 + l
  })
}

#[aoc(day5, part1)]
fn solve_part1(input: &str) -> u64 {
  solve(input).max().unwrap_or_default()
}

#[aoc(day5, part2)]
fn solve_part2(input: &str) -> u64 {
  let min = solve(input).min().unwrap_or_default();
  let max = solve(input).max().unwrap_or_default();
  (min..=max).fold(0, |s, v| s ^ v) ^ solve(input).fold(0, |s, v| s ^ v)
}
