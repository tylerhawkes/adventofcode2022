use std::{collections::HashMap, fmt::Display, ops::Index, str::FromStr};

pub fn compute(s: &str) -> (usize, usize) {
  let mut cave = s.parse::<Cave>().unwrap();
  let count = cave.run();
  cave.floor = true;
  let count2 = cave.run();
  (count, count + count2)
}

struct Cave {
  points: HashMap<(u16, u16), Grid>,
  max_y: u16,
  floor: bool,
}

impl Cave {
  fn run(&mut self) -> usize {
    'outer: for sand_units in 0.. {
      let mut x = 500;
      if !self[(x, 0)].is_air() {
        return sand_units;
      }
      for y in 0..self.max_y + 5 {
        if !self[(x, y)].is_air() {
          if self[(x - 1, y)].is_air() {
            x -= 1;
          } else if self[(x + 1, y)].is_air() {
            x += 1;
          } else {
            // println!("inserting at ({x}, {})", y - 1);
            self.points.insert((x, y - 1), Grid::Sand);
            continue 'outer;
          }
        }
      }
      return sand_units;
    }
    unreachable!()
  }
  fn print(&self) {
    println!("Cave: ");
    let min_x = self.points.keys().map(|c| c.0).min().unwrap();
    let max_x = self.points.keys().map(|c| c.0).max().unwrap();
    for y in 0..self.max_y + 5 {
      for x in min_x - 5..max_x + 5 {
        if (x, y) == (500, 0) {
          print!("X");
        } else {
          print!("{}", self[(x, y)]);
        }
      }
      println!();
    }
  }
}

impl Index<(u16, u16)> for Cave {
  type Output = Grid;

  fn index(&self, index: (u16, u16)) -> &Self::Output {
    static AIR: Grid = Grid::Air;
    static ROCK: Grid = Grid::Rock;
    self.points.get(&index).unwrap_or_else(|| {
      if self.floor && index.1 >= self.max_y + 2 {
        &ROCK
      } else {
        &AIR
      }
    })
  }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
enum Grid {
  #[default]
  Air,
  Rock,
  Sand,
}

impl Grid {
  const fn is_air(self) -> bool {
    matches!(self, Self::Air)
  }
}

impl Display for Grid {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Air => f.write_str(" "),
      Self::Rock => f.write_str("#"),
      Self::Sand => f.write_str("."),
    }
  }
}

impl FromStr for Cave {
  type Err = core::convert::Infallible;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut points = HashMap::new();
    for line in s.lines() {
      let mut connecting_coords = Vec::new();
      for coord in line.split(" -> ") {
        let mut split = coord.split(',');
        let x = split.next().unwrap().parse::<u16>().unwrap();
        let y = split.next().unwrap().parse::<u16>().unwrap();
        connecting_coords.push((x, y));
      }
      for line in connecting_coords.windows(2) {
        let start = line[0];
        let end = line[1];
        let mut added = 0;
        for x in start.0.min(end.0)..=start.0.max(end.0) {
          for y in start.1.min(end.1)..=start.1.max(end.1) {
            points.insert((x, y), Grid::Rock);
            added += 1;
          }
        }
      }
    }
    let max_y = points.keys().map(|(_, y)| *y).max().unwrap();
    Ok(Self {
      max_y,
      points,
      floor: false,
    })
  }
}
