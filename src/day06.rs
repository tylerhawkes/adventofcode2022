use std::collections::HashSet;

pub fn compute(s: &str) -> (usize, usize) {
  (find_consecutive_diffs(s, 4), find_consecutive_diffs(s, 14))
}

fn find_consecutive_diffs(s: &str, consecutive: usize) -> usize {
  let mut set = HashSet::new();
  s.as_bytes()
    .windows(consecutive)
    .position(|w| {
      set.clear();
      set.extend(w.iter().copied());
      set.len() == w.len()
    })
    .unwrap()
    + consecutive
}
