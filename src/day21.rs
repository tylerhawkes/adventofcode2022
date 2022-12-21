use std::{collections::HashMap, str::FromStr};

pub fn compute(s: &str) -> (u64, u64) {
  let mut map = s
    .lines()
    .map(|l| {
      let split = l.split(": ").collect::<Vec<_>>();
      (split[0].to_string(), split[1].parse::<Job>().unwrap())
    })
    .collect::<HashMap<_, _>>();
  let root_yell = map.get("root").unwrap().resolve(&map).unwrap();
  *map.get_mut("humn").unwrap() = Job::Yell(None);
  let root = map.get("root").unwrap();
  let Job::Compute(l, r, _, _) = map.get("root").unwrap() else {
    unreachable!()
  };
  let l_val = map.get(l).unwrap().resolve(&map);
  let r_val = map.get(r).unwrap().resolve(&map);
  let humn = match (l_val, r_val) {
    (Some(l), None) => map.get(r).unwrap().resolve_to(&map, l),
    (None, Some(r)) => map.get(l).unwrap().resolve_to(&map, r),
    _ => unreachable!(),
  };
  (root_yell, humn)
}

enum Job {
  Yell(Option<u64>),
  Compute(
    String,
    String,
    fn(u64, u64) -> u64,
    fn(Option<u64>, Option<u64>, u64) -> u64,
  ),
}

impl Job {
  fn resolve(&self, map: &HashMap<String, Job>) -> Option<u64> {
    match self {
      Self::Yell(u) => *u,
      Self::Compute(l, r, op, _rev) => {
        let l = map.get(l).unwrap().resolve(map);
        let r = map.get(r).unwrap().resolve(map);
        Some(op(l?, r?))
      }
    }
  }
  fn resolve_to(&self, map: &HashMap<String, Job>, e: u64) -> u64 {
    match self {
      Self::Yell(None) => e,
      Self::Yell(Some(_)) => unreachable!(),
      Self::Compute(l, r, op, rev) => {
        let l = map.get(l).unwrap();
        let r = map.get(r).unwrap();
        match (l.resolve(map), r.resolve(map)) {
          (Some(l), Some(r)) => unreachable!(),
          (Some(l), None) => {
            let expected = rev(Some(l), None, e);
            r.resolve_to(map, expected)
          }
          (None, Some(r)) => {
            let expected = rev(None, Some(r), e);
            l.resolve_to(map, expected)
          }
          _ => unreachable!(),
        }
      }
    }
  }
}

impl FromStr for Job {
  type Err = core::convert::Infallible;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let split = s.split_whitespace().collect::<Vec<_>>();
    match split.as_slice() {
      [i] => Ok(Self::Yell(Some(i.parse().unwrap()))),
      [l, op, r] => {
        let f = match *op {
          "-" => |l, r| l - r,
          "+" => |l, r| l + r,
          "*" => |l, r| l * r,
          "/" => |l, r| l / r,
          _ => unreachable!(),
        };
        let rev = match *op {
          "-" => |l, r, e| match (l, r) {
            (Some(l), None) => l - e,
            (None, Some(r)) => r + e,
            _ => unreachable!(),
          },
          "+" => |l, r, e| match (l, r) {
            (Some(l), None) => e - l,
            (None, Some(r)) => e - r,
            _ => unreachable!(),
          },
          "*" => |l, r, e| match (l, r) {
            (Some(l), None) => e / l,
            (None, Some(r)) => e / r,
            _ => unreachable!(),
          },
          "/" => |l, r, e| match (l, r) {
            (Some(l), None) => l / e,
            (None, Some(r)) => r * e,
            _ => unreachable!(),
          },
          _ => unreachable!(),
        };
        Ok(Self::Compute(l.to_string(), r.to_string(), f, rev))
      }
      _ => unreachable!(),
    }
  }
}
