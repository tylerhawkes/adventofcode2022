use std::collections::HashSet;

pub fn compute(s: &str) -> (usize, usize) {
  let priority_sum = s
    .lines()
    .map(|l| {
      let (l, r) = l.as_bytes().split_at(l.len() / 2);
      assert_eq!(l.len(), r.len());
      let l_set = l.iter().copied().collect::<HashSet<u8>>();
      let r_set = r.iter().copied().collect::<HashSet<u8>>();
      let common = l_set.intersection(&r_set).next().copied().unwrap();
      priority(common)
    })
    .sum::<usize>();
  let badge_priorities = s
    .lines()
    .collect::<Vec<_>>()
    .chunks(3)
    .map(|c| {
      assert_eq!(c.len(), 3);
      let e1 = c[0].as_bytes().iter().copied().collect::<HashSet<_>>();
      let e2 = c[1].as_bytes().iter().copied().collect::<HashSet<_>>();
      let e3 = c[2].as_bytes().iter().copied().collect::<HashSet<_>>();
      let common = e1
        .intersection(&e2)
        .copied()
        .collect::<HashSet<_>>()
        .intersection(&e3)
        .copied()
        .next()
        .unwrap();
      priority(common)
    })
    .sum::<usize>();
  (priority_sum, badge_priorities)
}

const fn priority(u: u8) -> usize {
  if (u as char).is_ascii_lowercase() {
    (u - b'a' + 1) as usize
  } else {
    (u - b'A' + 27) as usize
  }
}
