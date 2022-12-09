use std::collections::HashSet;

pub fn compute(s: &str) -> (usize, usize) {
  let movements = s
    .lines()
    .map(|s| {
      let split = s.split_whitespace().collect::<Vec<_>>();
      (
        split[0].chars().next().unwrap(),
        split[1].parse::<isize>().unwrap(),
      )
    })
    .collect::<Vec<_>>();

  (
    tail_positions(2, &movements),
    tail_positions(10, &movements),
  )
}

fn tail_positions(count: usize, movements: &[(char, isize)]) -> usize {
  let mut positions = vec![(0, 0); count];
  let mut tail_positions = HashSet::new();
  for (direction, steps) in movements.iter().copied() {
    for step in 0..steps {
      let head = positions[0];
      positions[0] = match direction {
        'U' => (head.0, head.1 + 1),
        'D' => (head.0, head.1 - 1),
        'L' => (head.0 - 1, head.1),
        'R' => (head.0 + 1, head.1),
        _ => unreachable!(),
      };
      for i in 1..positions.len() {
        positions[i] = move_tail(positions[i - 1], positions[i]);
      }
      tail_positions.insert(positions.last().copied().unwrap());
    }
  }
  tail_positions.len()
}

fn move_tail(head: (isize, isize), tail: (isize, isize)) -> (isize, isize) {
  let dist = (head.0 - tail.0, head.1 - tail.1);
  if dist.0.abs() <= 1 && dist.1.abs() <= 1 {
    tail
  } else {
    (tail.0 + dist.0.clamp(-1, 1), tail.1 + dist.1.clamp(-1, 1))
  }
}
