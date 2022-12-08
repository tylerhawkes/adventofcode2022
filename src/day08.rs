pub fn compute(s: &str) -> (usize, usize) {
  let trees = s
    .lines()
    .map(|l| {
      l.as_bytes()
        .iter()
        .copied()
        .map(|b| b - b'0')
        .collect::<Vec<u8>>()
    })
    .collect::<Vec<_>>();
  let forest = Forest::new(trees);
  (forest.visible(), forest.max_scenic_score())
}

struct Forest<T> {
  rows: usize,
  columns: usize,
  trees: Vec<Vec<T>>,
}

impl<T> Forest<T> {
  fn new(trees: Vec<Vec<T>>) -> Self {
    Self {
      rows: trees.len(),
      columns: trees[0].len(),
      trees,
    }
  }
  fn print(&self, f: impl Fn(&T) -> char) {
    for row in &self.trees {
      for column in row {
        print!("{}", f(column));
      }
      println!();
    }
  }
  fn map<U>(&self, f: impl Fn(&T, usize, usize) -> U) -> Forest<U> {
    Forest {
      columns: self.columns,
      rows: self.rows,
      trees: self
        .trees
        .iter()
        .enumerate()
        .map(|(row, v)| {
          v.iter()
            .enumerate()
            .map(|(column, t)| f(t, row, column))
            .collect::<Vec<U>>()
        })
        .collect::<Vec<_>>(),
    }
  }
}
impl<T: Ord + Copy> Forest<T> {
  fn visible(&self) -> usize {
    let mut visible = self.map(|_, row, column| self.is_visible(row, column));
    #[cfg(debug_assertions)]
    visible.print(|b| if *b { '#' } else { '.' });
    visible
      .trees
      .iter()
      .map(|r| r.iter().filter(|x| **x).count())
      .sum()
  }
  fn is_visible(&self, row: usize, column: usize) -> bool {
    let value = self.trees[row][column];
    // left
    self.trees[row][..column].iter().all(|t| t < &value)
    // right
    || self.trees[row][column + 1..].iter().all(|t| t < &value)
    // top
    || self.trees[..row].iter().all(|c| c[column] < value)
    // bottom
    || self.trees[row + 1..].iter().all(|c| c[column] < value)
  }
  fn max_scenic_score(&self) -> usize {
    let scenic_scores = self.map(|_, row, column| self.scenic_score(row, column));
    scenic_scores
      .trees
      .iter()
      .map(|row| row.iter().copied().max().unwrap())
      .max()
      .unwrap()
  }
  fn scenic_score(&self, row: usize, column: usize) -> usize {
    let value = self.trees[row][column];
    self.trees[row][..column].iter().rev().fold((true, 0), |(limit, count), t|{
      if limit {
        (t < &value, count + 1)
      } else {
        (false, count)
      }
    }).1
    // right
    * self.trees[row][column + 1..].iter().fold((true, 0), |(limit, count), t|{
      if limit {
        (t < &value, count + 1)
      } else {
        (false, count)
      }
    }).1
    // top
    * self.trees[..row].iter().rev().fold((true, 0), |(limit, count), t|{
      if limit {
        (t[column] < value, count + 1)
      } else {
        (false, count)
      }
    }).1
    // bottom
    * self.trees[row + 1..].iter().fold((true, 0), |(limit, count), t|{
      if limit {
        (t[column] < value, count + 1)
      } else {
        (false, count)
      }
    }).1
  }
}
