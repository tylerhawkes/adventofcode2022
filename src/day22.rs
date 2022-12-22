use std::{collections::HashMap, str::FromStr};

// not 13226
pub fn compute(s: &str) -> (usize, usize) {
  let map = s.parse::<Map>().unwrap();
  let mut direction = Direction::Right;
  let (mut position, c) = direction.next(map.min, &map);
  assert!(c);
  let mut seen = HashMap::new();
  for mv in &map.moves {
    println!("Move: {mv:?}");
    match mv {
      Move::Left => direction = direction.turn_left(),
      Move::Right => direction = direction.turn_right(),
      Move::Forward(steps) => {
        println!("{position:?}, {direction:?}");
        seen.insert(position, direction);
        // println!("moving {steps} steps at {position:?}, {direction:?}");
        for step in 0..*steps {
          let kontinue;
          (position, kontinue) = direction.next(position, &map);
          seen.insert(position, direction);
          if !kontinue {
            // println!("at rock {position:?}");
            break;
          }
        }
      }
    }
  }
  map.print(&seen);
  println!("{position:?}, {direction:?}");
  (
    1000 * position.1 as usize + 4 * position.0 as usize + direction.facing_score(),
    0,
  )
}

type Coord = (i16, i16);

#[derive(Copy, Clone, Debug)]
enum Direction {
  Up,
  Down,
  Left,
  Right,
}

impl Direction {
  fn to_char(self) -> char {
    match self {
      Self::Up => '^',
      Self::Down => 'v',
      Self::Left => '<',
      Self::Right => '>',
    }
  }
  fn facing_score(self) -> usize {
    match self {
      Self::Up => 3,
      Self::Down => 1,
      Self::Left => 2,
      Self::Right => 0,
    }
  }
  fn turn_left(self) -> Self {
    match self {
      Self::Up => Self::Left,
      Self::Down => Self::Right,
      Self::Left => Self::Down,
      Self::Right => Self::Up,
    }
  }
  fn turn_right(self) -> Self {
    match self {
      Self::Up => Self::Right,
      Self::Down => Self::Left,
      Self::Left => Self::Up,
      Self::Right => Self::Down,
    }
  }
  fn next(self, original_position: Coord, map: &Map) -> (Coord, bool) {
    let mut position = original_position;
    let delta = match self {
      Self::Up => (0, -1),
      Self::Down => (0, 1),
      Self::Left => (-1, 0),
      Self::Right => (1, 0),
    };
    let mut last_none = false;
    loop {
      let next_position = (position.0 + delta.0, position.1 + delta.1);
      let next = map.map.get(&next_position);
      // println!("{position:?}, {next_position:?}, {next:?}");
      match next {
        Some(Tile::Space) => return (next_position, true),
        Some(Tile::Rock) => return (original_position, false),
        None => {
          last_none = true;
          position = match self {
            Self::Up => {
              if next_position.1 < map.min.1 {
                (next_position.0, map.max.1 + 1)
              } else {
                next_position
              }
            }
            Self::Down => {
              if next_position.1 > map.max.1 {
                (next_position.0, map.min.1 - 1)
              } else {
                next_position
              }
            }
            Self::Left => {
              if next_position.0 < map.min.0 {
                (map.max.0 + 1, next_position.1)
              } else {
                next_position
              }
            }
            Self::Right => {
              if next_position.0 > map.max.0 {
                (map.min.0 - 1, next_position.1)
              } else {
                next_position
              }
            }
          }
        }
      }
    }
  }
}

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
enum Tile {
  Space,
  Rock,
}

#[derive(Copy, Clone, Debug)]
enum Move {
  Forward(i16),
  Left,
  Right,
}

struct Map {
  map: HashMap<Coord, Tile>,
  moves: Vec<Move>,
  min: Coord,
  max: Coord,
}

impl Map {
  fn print(&self, seen: &HashMap<Coord, Direction>) {
    for row in self.min.1..=self.max.1 {
      for column in self.min.0..=self.max.0 {
        let c = (column, row);
        let to_print = if let Some(direction) = seen.get(&c) {
          direction.to_char()
        } else {
          match self.map.get(&c) {
            Some(Tile::Rock) => '#',
            Some(Tile::Space) => '.',
            None => ' ',
          }
        };
        print!("{}", to_print);
      }
      println!();
    }
  }
}

impl FromStr for Map {
  type Err = core::convert::Infallible;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let split = s.split("\n\n").collect::<Vec<_>>();
    let mut map = HashMap::new();
    for (y, row) in split[0].lines().enumerate() {
      for (x, column) in row.as_bytes().iter().copied().enumerate() {
        match column {
          b' ' => None,
          b'.' => map.insert((x as i16 + 1, y as i16 + 1), Tile::Space),
          b'#' => map.insert((x as i16 + 1, y as i16 + 1), Tile::Rock),
          _ => unreachable!(),
        };
      }
    }
    let mut last_byte = 0_i16;
    let mut moves = vec![];
    for byte in split[1].as_bytes().iter().copied() {
      match byte {
        b'0'..=b'9' => last_byte = last_byte * 10 + (byte - b'0') as i16,
        b'L' => {
          moves.push(Move::Forward(last_byte));
          last_byte = 0;
          moves.push(Move::Left);
        }
        b'R' => {
          moves.push(Move::Forward(last_byte));
          last_byte = 0;
          moves.push(Move::Right);
        }
        _ => unreachable!(),
      }
    }
    moves.push(Move::Forward(last_byte));
    let min = map
      .keys()
      .fold((i16::MAX, i16::MAX), |l, r| (l.0.min(r.0), l.1.min(r.1)));
    let max = map
      .keys()
      .fold((i16::MIN, i16::MIN), |l, r| (l.0.max(r.0), l.1.max(r.1)));
    Ok(Self {
      map,
      moves,
      min,
      max,
    })
  }
}

#[test]
fn test_22() {
  let input = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";
  dbg!(compute(input));
  panic!();
}
