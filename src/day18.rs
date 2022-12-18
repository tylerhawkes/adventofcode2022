use std::{collections::HashSet, str::FromStr};

pub fn compute(s: &str) -> (usize, usize) {
  let cubes = s.parse::<Cubes>().unwrap();
  let mut faces = 0;
  for cube in cubes.cubes.iter().copied() {
    faces += cube
      .neighbors()
      .into_iter()
      .filter(|n| !cubes.cubes.contains(n))
      .count();
  }
  let mut outside_faces = 0;
  let mut inside_cubes = HashSet::with_capacity(2000);
  let min = cubes.min_coord;
  let max = cubes.max_coord;
  for x in min.x..=max.x {
    for y in min.y..=max.y {
      for z in min.z..=max.z {
        let c = Coord3D { x, y, z };
        let path = pathfinding::directed::bfs::bfs(
          &c,
          |c| {
            c.neighbors()
              .into_iter()
              .filter(|n| !cubes.cubes.contains(n))
          },
          |c| {
            c.x <= min.x
              || c.x >= max.x
              || c.y <= min.y
              || c.y >= max.y
              || c.z <= min.z
              || c.z >= max.z
          },
        );
        if path.is_none() {
          inside_cubes.insert(c);
        }
      }
    }
  }
  for cube in cubes.cubes.iter().copied() {
    outside_faces += cube
      .neighbors()
      .into_iter()
      .filter(|n| !cubes.cubes.contains(n) && !inside_cubes.contains(n))
      .count();
  }
  (faces, outside_faces)
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Coord3D {
  x: i16,
  y: i16,
  z: i16,
}

impl Coord3D {
  fn neighbors(self) -> [Self; 6] {
    [
      self.diff_x(-1),
      self.diff_x(1),
      self.diff_y(-1),
      self.diff_y(1),
      self.diff_z(-1),
      self.diff_z(1),
    ]
  }
  // fn outside_faces(
  //   self,
  //   min_coord: Coord3D,
  //   max_coord: Coord3D,
  // ) -> [Box<dyn Iterator<Item = Self>>; 6] {
  //   [
  //     Box::new((min_coord.x..self.x).rev().map(move |o| self.diff_x(o))),
  //     Box::new((self.x + 1..=max_coord.x).map(move |o| self.diff_x(o))),
  //     Box::new((min_coord.y..self.y).rev().map(move |o| self.diff_y(o))),
  //     Box::new((self.y + 1..=max_coord.y).map(move |o| self.diff_y(o))),
  //     Box::new((min_coord.z..self.z).rev().map(move |o| self.diff_z(o))),
  //     Box::new((self.z + 1..=max_coord.z).map(move |o| self.diff_z(o))),
  //   ]
  // }
  fn diff_x(self, diff: i16) -> Self {
    Self {
      x: self.x + diff,
      ..self
    }
  }
  fn diff_y(self, diff: i16) -> Self {
    Self {
      y: self.y + diff,
      ..self
    }
  }
  fn diff_z(self, diff: i16) -> Self {
    Self {
      z: self.z + diff,
      ..self
    }
  }
}

struct Cubes {
  cubes: HashSet<Coord3D>,
  min_coord: Coord3D,
  max_coord: Coord3D,
}

impl FromStr for Cubes {
  type Err = core::convert::Infallible;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let cubes = s
      .lines()
      .map(|l| {
        let c = l
          .split(',')
          .map(|s| s.parse::<i16>().unwrap())
          .collect::<Vec<_>>();
        Coord3D {
          x: c[0],
          y: c[1],
          z: c[2],
        }
      })
      .collect::<HashSet<_>>();
    let min_coord = cubes
      .iter()
      .fold(
        Coord3D {
          x: i16::MAX,
          z: i16::MAX,
          y: i16::MAX,
        },
        |l, r| Coord3D {
          x: l.x.min(r.x),
          y: l.y.min(r.y),
          z: l.z.min(r.z),
        },
      )
      .diff_x(-1)
      .diff_y(-1)
      .diff_z(-1);
    let max_coord = cubes
      .iter()
      .fold(
        Coord3D {
          x: i16::MIN,
          z: i16::MIN,
          y: i16::MIN,
        },
        |l, r| Coord3D {
          x: l.x.max(r.x),
          y: l.y.max(r.y),
          z: l.z.max(r.z),
        },
      )
      .diff_x(1)
      .diff_y(1)
      .diff_z(1);
    Ok(Self {
      cubes,
      min_coord,
      max_coord,
    })
  }
}

#[test]
fn test_18() {
  let input = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";
  dbg!(compute(input));
  panic!();
}
