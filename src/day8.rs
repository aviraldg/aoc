use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
enum Instruction {
  Nop { value: i64 },
  Acc { value: i64 },
  Jmp { offset: i64 },
}

impl Instruction {
  fn apply(self: &Self, state: &State) -> State {
    let mut state = state.clone();
    match self {
      Instruction::Nop { value: _ } => {}
      Instruction::Acc { value } => {
        state.acc += value;
      }
      Instruction::Jmp { offset } => {
        state.pc += offset - 1;
      }
    }
    state
  }

  fn flip(self: &Self) -> Self {
    match self {
      Instruction::Nop { value } => Instruction::Jmp { offset: *value },
      Instruction::Jmp { offset } => Instruction::Nop { value: *offset },
      _ => *self,
    }
  }
}

#[derive(Debug, Default, Clone, Copy)]
struct State {
  pc: i64,
  acc: i64,
}

impl FromStr for Instruction {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let parts: Vec<&str> = s.split(" ").collect();
    if parts.len() != 2 {
      panic!();
    }

    let value = parts[1].parse::<i64>();
    if value.is_err() {
      return Err(());
    }
    let value = value.unwrap();

    match parts[0] {
      "nop" => Ok(Instruction::Nop { value }),
      "acc" => Ok(Instruction::Acc { value }),
      "jmp" => Ok(Instruction::Jmp { offset: value }),
      _ => Err(()),
    }
  }
}

#[aoc_generator(day8)]
fn input_generator(input: &str) -> Vec<Instruction> {
  input
    .lines()
    .map(Instruction::from_str)
    .map(|i_or| i_or.unwrap())
    .collect()
}

#[aoc(day8, part1)]
fn solve_part1(input: &Vec<Instruction>) -> i64 {
  let mut state = State::default();
  let mut visited: Vec<bool> = vec![false; input.len()];

  while (state.pc as usize) < input.len() {
    if visited[state.pc as usize] {
      break;
    }
    visited[state.pc as usize] = true;
    let instruction = &input[state.pc as usize];
    state.pc += 1;
    state = instruction.apply(&mut state);
  }

  state.acc
}

#[aoc(day8, part2)]
fn solve_part2(input: &Vec<Instruction>) -> i64 {
  let instructions = input.clone();
  let mut state = State::default();
  let mut flip = false;
  let mut visited: Vec<i64> = vec![i64::MAX; instructions.len()];

  while (state.pc as usize) < instructions.len() {
    dbg!(state, flip, &visited);
    if visited[state.pc as usize] < state.pc {
      state = State::default();
      flip = false;
    }
    visited[state.pc as usize] = state.pc;
    let instruction = &instructions[state.pc as usize];
    state.pc += 1;

    let is_acc = if let Instruction::Acc { value: _ } = instruction {
      true
    } else {
      false
    };

    let new_flip_state = instruction.flip().apply(&state);
    let new_state = instruction.apply(&state);
    dbg!(new_flip_state, new_state);
    let flipped = if !flip && !is_acc && visited[new_flip_state.pc as usize] == i64::MAX {
      flip = true;
      state = new_flip_state;
      true
    } else {
      state = new_state;
      false
    };
    dbg!(flipped);
  }

  state.acc
}
