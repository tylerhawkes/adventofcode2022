use std::str::FromStr;

pub fn compute(s: &str) -> (String, String) {
  let mut lines = s.lines();
  let mut stacks = parse_stacks(&mut lines);
  let mut stacks2 = stacks.clone();
  let moves = parse_moves(&mut lines);
  let mut transient = vec![];
  for mv in moves {
    transient.clear();
    for _ in 0..mv.count {
      let container = stacks[mv.from].pop().unwrap();
      stacks[mv.to].push(container);
      let container = stacks2[mv.from].pop().unwrap();
      transient.push(container);
    }
    transient
      .iter()
      .rev()
      .copied()
      .for_each(|c| stacks2[mv.to].push(c));
  }
  let arrangement = stacks
    .iter()
    .map(|s| s.last().copied().unwrap_or(' '))
    .collect::<String>();
  let arrangement2 = stacks2
    .iter()
    .map(|s| s.last().copied().unwrap_or(' '))
    .collect::<String>();
  (arrangement, arrangement2)
}

fn parse_stacks<'a, I: Iterator<Item = &'a str>>(i: &mut I) -> Vec<Vec<char>> {
  let mut stacks = vec![vec![]; 9];
  for line in i {
    if line.is_empty() {
      stacks.iter_mut().for_each(|v| v.reverse());
      return stacks;
    }
    for (i, c) in line.chars().enumerate() {
      if c.is_ascii_uppercase() {
        stacks[(i - 1) / 4].push(c);
      }
    }
  }
  unreachable!()
}

fn parse_moves<'a, I: Iterator<Item = &'a str>>(i: &mut I) -> Vec<Move> {
  let mut moves = vec![];
  for line in i {
    moves.push(line.parse().unwrap());
  }
  moves
}

#[derive(Debug)]
struct Move {
  count: usize,
  from: usize,
  to: usize,
}

impl FromStr for Move {
  type Err = std::convert::Infallible;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let split = s.split_whitespace().collect::<Vec<_>>();
    let count = split[1].parse().unwrap();
    let from = split[3].parse::<usize>().unwrap() - 1;
    let to = split[5].parse::<usize>().unwrap() - 1;
    Ok(Move { count, from, to })
  }
}
