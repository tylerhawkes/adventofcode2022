use std::collections::{HashMap, HashSet};

pub fn compute(s: &str) -> (usize, usize) {
  let mut chamber = Chamber {
    rocks: HashSet::with_capacity(2000),
    highest_point: 0,
    jets: s
      .as_bytes()
      .iter()
      .copied()
      .map(|b| match b {
        b'<' => Jet::Left,
        b'>' => Jet::Right,
        _ => unreachable!(),
      })
      .collect(),
    steps: 0,
    stop_step: 2022,
    additional_highest: 0,
  };
  for i in -2..9 {
    chamber.rocks.insert((i, 0));
  }
  let part_1 = chamber.clone().run();
  chamber.stop_step = 1000000000000;
  let part_2 = chamber.run();
  (part_1, part_2)
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
enum Rocks {
  Minus,
  Plus,
  L,
  Pipe,
  Square,
}

type C = (i32, i32);

impl Rocks {
  fn shape(self) -> &'static Shape {
    match self {
      Self::Minus => &Shape([(0, 0), (1, 0), (2, 0), (3, 0)], None),
      Self::Plus => &Shape([(1, 0), (0, 1), (1, 1), (2, 1)], Some((1, 2))),
      Self::L => &Shape([(0, 0), (1, 0), (2, 0), (2, 1)], Some((2, 2))),
      Self::Pipe => &Shape([(0, 0), (0, 1), (0, 2), (0, 3)], None),
      Self::Square => &Shape([(0, 0), (1, 0), (0, 1), (1, 1)], None),
    }
  }
  fn iter() -> impl Iterator<Item = Self> + Clone {
    [Self::Minus, Self::Plus, Self::L, Self::Pipe, Self::Square].into_iter()
  }
}

struct Shape([C; 4], Option<C>);

impl Shape {
  fn intersects_any(&self, c: C, rocks: &HashSet<C>) -> bool {
    self
      .iter(c)
      .any(|c2| c2.0 < 0 || c2.0 > 6 || rocks.contains(&c2))
  }
  fn iter(&self, c: C) -> impl Iterator<Item = C> + '_ {
    self
      .0
      .iter()
      .copied()
      .chain(self.1.into_iter())
      .map(move |c2| (c2.0 + c.0, c2.1 + c.1))
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
enum Jet {
  Left,
  Right,
}

impl Jet {
  fn mv(self) -> i32 {
    match self {
      Self::Left => -1,
      Self::Right => 1,
    }
  }
}

#[derive(Clone)]
struct Chamber {
  rocks: HashSet<C>,
  highest_point: i32,
  jets: Vec<Jet>,
  steps: usize,
  stop_step: usize,
  additional_highest: usize,
}

impl Chamber {
  fn run(&mut self) -> usize {
    let jet_len = self.jets.len();
    // println!("{jet_len}");
    let mut repeating = HashMap::<(Rocks, usize), Vec<(usize, usize, usize)>>::new();
    let jets = self.jets.clone();
    let mut jets = jets.iter().copied().cycle();
    let mut moves = 1;
    for rock in Rocks::iter().cycle() {
      let mut rock_position = (2, self.highest_point + 4);
      for mv in 1.. {
        // println!("{rock_position:?}");
        let shape = rock.shape();
        let next_position = (rock_position.0 + jets.next().unwrap().mv(), rock_position.1);
        if !shape.intersects_any(next_position, &self.rocks) {
          rock_position = next_position;
        }
        moves += 1;
        let next_position = (rock_position.0, rock_position.1 - 1);
        if mv > 2 && shape.intersects_any(next_position, &self.rocks) {
          for c in shape.iter(rock_position) {
            self.highest_point = self.highest_point.max(c.1);
            self.rocks.insert(c);
          }
          // println!(
          //   "moves: {moves}, rock: {rock:0>6?}, rem: {}, additional_highest: {}, highest: {}, stop_step: {}, steps: {}",
          //   moves % jet_len, self.additional_highest, self.highest_point, self.stop_step, self.steps
          // );
          if self.additional_highest == 0 {
            let vec = repeating.entry((rock, moves % jet_len)).or_default();
            // throw the first one out since it isn't in the pattern
            if vec.first().map_or(0, |v| v.1) < jet_len {
              vec.pop();
            }
            vec.push((self.highest_point as usize, moves, self.steps));

            // this can be anything greater than 2 if you want to make sure you're observing a pattern
            if vec.len() >= 2 {
              // println!("{vec:?}");
              let first = vec[vec.len() - 2];
              let next = vec[vec.len() - 1];
              let step_groups = (self.stop_step - self.steps) / (next.2 - first.2);
              self.stop_step = 1000000000000;
              self.additional_highest = step_groups * (next.0 - first.0);
              self.steps += step_groups * (next.2 - first.2);
              // println!(
              //   "additional_highest: {}, highest: {}, stop_step: {}, steps: {}",
              //   self.additional_highest, self.highest_point, self.stop_step, self.steps
              // );
            }
          }
          // repeats every 1725 steps
          // raises height 2728 every repeat
          // found pattern at step 19265 and height 30398 using 10 steps
          self.steps += 1;
          if self.steps >= self.stop_step {
            return self.highest_point as usize + self.additional_highest;
          }
          break;
        } else {
          rock_position = next_position;
        }
      }
    }
    self.highest_point as usize
  }
  fn print(&self) {
    for row in (self.highest_point - 20..=self.highest_point).rev() {
      for col in -1..8 {
        if col == -1 || col == 7 {
          print!("|");
        } else if self.rocks.contains(&(col, row)) {
          print!("#");
        } else {
          print!(".");
        }
      }
      println!();
    }
  }
}
