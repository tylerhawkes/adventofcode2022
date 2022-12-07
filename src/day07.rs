use std::{collections::BTreeMap, str::FromStr};

pub fn compute(s: &str) -> (usize, usize) {
  let dir = s.parse::<Directory>().unwrap();
  (
    dir.total_sizes_under_n(100_000),
    dir.smallest_size_over_n(dir.total_size - 40_000_000),
  )
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Directory {
  total_size: usize,
  nodes: BTreeMap<String, Node>,
}

impl Directory {
  fn dirs(&self) -> impl Iterator<Item = &Self> {
    self
      .nodes
      .values()
      .filter_map(|n| if let Node::Dir(d) = n { Some(d) } else { None })
  }
  fn compute_size(&mut self) -> usize {
    let mut size = 0;
    for node in self.nodes.values_mut() {
      match node {
        Node::File(u) => size += *u,
        Node::Dir(dir) => size += dir.compute_size(),
      }
    }
    self.total_size = size;
    size
  }
  fn total_sizes_under_n(&self, n: usize) -> usize {
    let mut size = if self.total_size < n {
      self.total_size
    } else {
      0
    };
    for d in self.dirs() {
      size += d.total_sizes_under_n(n);
    }
    size
  }
  fn smallest_size_over_n(&self, n: usize) -> usize {
    let mut smallest = if self.total_size >= n {
      self.total_size
    } else {
      usize::MAX
    };
    for d in self.dirs() {
      if d.total_size >= n {
        smallest = smallest.min(d.smallest_size_over_n(n));
      }
    }
    smallest
  }
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Node {
  Dir(Directory),
  File(usize),
}

impl Node {
  fn unwrap_dir(&mut self) -> &mut Directory {
    match self {
      Self::Dir(dir) => dir,
      Self::File(_) => unreachable!(),
    }
  }
}

impl FromStr for Directory {
  type Err = ::core::convert::Infallible;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut base = Directory {
      total_size: 0,
      nodes: BTreeMap::new(),
    };
    let mut current_dir = &mut base;
    let mut dir_stack = vec![];
    for line in s.lines() {
      match line {
        "$ cd /" => {
          dir_stack.clear();
          current_dir = &mut base
        }
        "$ cd .." => {
          dir_stack.pop().unwrap();
          current_dir = &mut base;
          for dir in &dir_stack {
            current_dir = current_dir.nodes.get_mut(dir).unwrap().unwrap_dir();
          }
        }
        cd if cd.starts_with("$ cd ") => {
          let dir_name = cd[5..].to_string();
          current_dir = current_dir.nodes.get_mut(&dir_name).unwrap().unwrap_dir();
          dir_stack.push(dir_name);
        }
        "$ ls" => {}
        dir if dir.starts_with("dir") => {
          current_dir.nodes.insert(
            dir[4..].to_string(),
            Node::Dir(Directory {
              total_size: 0,
              nodes: BTreeMap::new(),
            }),
          );
        }
        file => {
          let parts = file.split_whitespace().collect::<Vec<_>>();
          assert_eq!(parts.len(), 2);
          current_dir
            .nodes
            .insert(parts[1].to_string(), Node::File(parts[0].parse().unwrap()));
        }
      }
    }
    base.compute_size();
    Ok(base)
  }
}
