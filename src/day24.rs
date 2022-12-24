#![warn(unused)]
use std::{
  ops::{Index, IndexMut},
  str::FromStr,
};

pub fn compute(s: &str) -> (usize, usize) {
  let valley = s.parse::<Valley>().unwrap();
  let mut valleys = vec![valley];
  loop {
    let next = valleys.last().unwrap().step();
    if &next == valleys.first().unwrap() {
      break;
    }
    valleys.push(next);
  }
  let (min, max) = (valleys[0].min, valleys[0].max);
  let dist1 = find_path_dist(&valleys, min, max, 0);
  let dist2 = find_path_dist(&valleys, max, min, dist1 + 1);
  let dist3 = find_path_dist(&valleys, min, max, dist2 + 1);
  (dist1, dist3)
}

fn find_path_dist(valleys: &[Valley], start: Coord, end: Coord, steps_taken: usize) -> usize {
  for i in steps_taken..steps_taken + valleys.len() {
    if valleys[i % valleys.len()][start] != Blizzard(0) {
      continue;
    }
    let path = pathfinding::directed::bfs::bfs(
      &(start, i),
      |&(c, i)| {
        valleys[(i + 1) % valleys.len()]
          .next_coords(c)
          .map(move |c| (c, i + 1))
      },
      |(c, _)| *c == end,
    );
    if let Some(path) = path {
      return path.len() + i;
    }
  }
  unreachable!()
}

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
enum Direction {
  North = 0,
  South = 1,
  East = 2,
  West = 3,
}

impl Direction {
  fn iter() -> impl Iterator<Item = Self> {
    [Self::East, Self::South, Self::North, Self::West].into_iter()
  }
  fn new_coord(self, c: Coord) -> Coord {
    match self {
      Self::North => (c.0, c.1 - 1),
      Self::South => (c.0, c.1 + 1),
      Self::East => (c.0 + 1, c.1),
      Self::West => (c.0 - 1, c.1),
    }
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(transparent)]
struct Blizzard(u8);

impl Blizzard {
  fn directions(self) -> impl Iterator<Item = Direction> {
    Direction::iter().filter(move |d| 1 << (*d as u8) & self.0 > 0)
  }
}

type Coord = (u16, u16);

#[derive(PartialEq, Eq)]
struct Valley {
  blizzards: Vec<Vec<Blizzard>>,
  start: Coord,
  end: Coord,
  min: Coord,
  max: Coord,
}

impl Valley {
  fn next_coords(&self, coord: Coord) -> impl Iterator<Item = Coord> + '_ {
    Direction::iter()
      .map(move |d| d.new_coord(coord))
      .chain([coord])
      .filter(|c| self[*c] == Blizzard(0))
  }
  fn step(&self) -> Self {
    let mut this = Self {
      blizzards: self
        .blizzards
        .iter()
        .map(|r| r.iter().map(|_| Blizzard(0)).collect())
        .collect(),
      start: self.start,
      end: self.end,
      min: self.min,
      max: self.max,
    };
    for f in this.blizzards.first_mut().unwrap() {
      *f = Blizzard(128);
    }
    for f in this.blizzards.last_mut().unwrap() {
      *f = Blizzard(128);
    }
    for row in &mut this.blizzards {
      *row.first_mut().unwrap() = Blizzard(128);
      *row.last_mut().unwrap() = Blizzard(128);
    }
    for y in 1..self.blizzards.len() - 1 {
      for x in 1..self.blizzards[0].len() - 1 {
        let coord = (x as u16, y as u16);
        for direction in self[coord].directions() {
          let new_coord = direction.new_coord(coord);
          // println!("{coord:?}, {direction:?}, {new_coord:?}");
          this[new_coord].0 |= 1 << (direction as u8);
        }
      }
    }
    this
  }
}

impl Index<Coord> for Valley {
  type Output = Blizzard;

  fn index(&self, index: Coord) -> &Self::Output {
    &self.blizzards[index.1 as usize][index.0 as usize]
  }
}

// funny business here makes it so that indices wrap around when setting values.
impl IndexMut<Coord> for Valley {
  fn index_mut(&mut self, index: Coord) -> &mut Self::Output {
    let index = if index.0 < self.min.0 {
      (self.max.0, index.1)
    } else if index.0 > self.max.0 {
      (self.min.0, index.1)
    } else if index.1 < self.min.1 {
      (index.0, self.max.1)
    } else if index.1 > self.max.1 {
      (index.0, self.min.1)
    } else {
      index
    };
    // println!("mut {index:?}");
    &mut self.blizzards[index.1 as usize][index.0 as usize]
  }
}

impl FromStr for Valley {
  type Err = core::convert::Infallible;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut blizzards = vec![];
    for line in s.lines() {
      let mut row = vec![];
      for col in line.as_bytes() {
        let b = match *col {
          b'#' => Blizzard(128),
          b'^' => Blizzard(1 << Direction::North as u8),
          b'v' => Blizzard(1 << Direction::South as u8),
          b'>' => Blizzard(1 << Direction::East as u8),
          b'<' => Blizzard(1 << Direction::West as u8),
          b'.' => Blizzard(0),
          _ => panic!("unrecognized value {col} ({})", *col as char),
        };
        row.push(b);
      }
      blizzards.push(row);
    }
    let start = blizzards
      .first()
      .unwrap()
      .iter()
      .position(|b| *b == Blizzard(0))
      .unwrap();
    let end = blizzards
      .last()
      .unwrap()
      .iter()
      .position(|b| *b == Blizzard(0))
      .unwrap();
    let start = (start as u16, 0);
    let end = (end as u16, blizzards.len() as u16 - 1);
    let mut valley = Valley {
      start,
      end,
      min: (1, 1),
      max: (blizzards[0].len() as u16 - 2, blizzards.len() as u16 - 2),
      blizzards,
    };
    valley.blizzards[start.1 as usize][start.0 as usize] = Blizzard(128);
    valley.blizzards[end.1 as usize][end.0 as usize] = Blizzard(128);
    Ok(valley)
  }
}
