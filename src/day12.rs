use std::{
  collections::{BinaryHeap, HashSet, VecDeque},
  ops::Index,
  str::FromStr,
};

pub fn compute(s: &str) -> (u32, u32) {
  let elevation = s.parse::<Elevation>().unwrap();
  let count = elevation.bfs();
  let mut min_a = count;
  for y in 0..elevation.heights.len() {
    for x in 0..elevation.heights[0].len() {
      if elevation[(x, y)] == 0 && elevation.moves((x, y)).any(|m| elevation[m.0] == 1) {
        if let Some((_, count)) = pathfinding::directed::dijkstra::dijkstra(
          &(x, y),
          |coord| elevation.moves(*coord),
          |coord| coord == &elevation.end,
        ) {
          if count < min_a {
            min_a = count;
          }
        }
      }
    }
  }
  (count, min_a)
}

struct Elevation {
  start: (usize, usize),
  end: (usize, usize),
  heights: Vec<Vec<u8>>,
}

impl Elevation {
  fn print(&self) {
    println!("start: {:?}, end: {:?}", self.start, self.end);
    for row in &self.heights {
      for column in row {
        print!("{}", (*column + b'a') as char);
      }
      println!();
    }
  }
  fn moves(&self, (x, y): (usize, usize)) -> impl Iterator<Item = ((usize, usize), u32)> + '_ {
    let height = self[(x, y)] + 1;
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
      .into_iter()
      .filter_map(move |(dx, dy)| {
        let (nx, ny) = (x as isize + dx, y as isize + dy);
        if nx < 0
          || nx >= self.heights[0].len() as isize
          || ny < 0
          || ny >= self.heights.len() as isize
        {
          None
        } else {
          Some(((nx as usize, ny as usize), 1))
        }
      })
      .filter(move |(coord, _)| self[*coord] <= height)
  }
  fn bfs(&self) -> u32 {
    let mut visited = HashSet::new();
    let mut to_visit = VecDeque::new();
    to_visit.push_back((self.start, 0));
    visited.insert(self.start);
    while !to_visit.is_empty() {
      let (next, count) = to_visit.pop_front().unwrap();
      for mv in self.moves(next) {
        if visited.insert(mv.0) {
          if mv.0 == self.end {
            return count + 1;
          } else {
            to_visit.push_back((mv.0, count + mv.1));
          }
        }
      }
    }
    unreachable!()
  }
}

impl Index<(usize, usize)> for Elevation {
  type Output = u8;

  fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
    &self.heights[y][x]
  }
}

impl FromStr for Elevation {
  type Err = core::convert::Infallible;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut start = None;
    let mut end = None;
    s.lines().enumerate().for_each(|(y, l)| {
      l.bytes().enumerate().for_each(|(x, b)| {
        if b == b'S' {
          start = (x, y).into();
        } else if b == b'E' {
          end = (x, y).into();
        }
      })
    });
    let start = start.unwrap();
    let end = end.unwrap();
    let mut heights = s
      .lines()
      .map(|l| {
        l.as_bytes()
          .iter()
          .copied()
          .map(|b| b.saturating_sub(b'a'))
          .collect::<Vec<_>>()
      })
      .collect::<Vec<_>>();
    heights[start.1][start.0] = 0;
    heights[end.1][end.0] = b'z' - b'a';
    Ok(Self {
      start,
      end,
      heights,
    })
  }
}
