use std::collections::{BTreeMap, HashMap, LinkedList};
// higher than 1623178306
pub fn compute(s: &str) -> (i64, i64) {
  let encrypted = s
    .lines()
    .map(|l| l.parse::<i64>().unwrap())
    .collect::<Vec<_>>();
  let encrypted2 = encrypted.iter().map(|e| *e * 811589153).collect::<Vec<_>>();

  (mix(&encrypted, 1), mix(&encrypted2, 10))
}

fn mix(encrypted: &[i64], times: usize) -> i64 {
  let len = encrypted.len();
  let mut encrypted_ref = encrypted.iter().collect::<Vec<_>>();
  for _ in 0..times {
    for i in encrypted {
      let mut position = encrypted_ref
        .iter()
        .position(|e| std::ptr::eq(*e, i))
        .unwrap();
      for _ in 0..(i.unsigned_abs() % (len as u64 - 1)) {
        let next_position = match i.cmp(&0) {
          std::cmp::Ordering::Greater => {
            if position + 1 == len {
              0
            } else {
              position + 1
            }
          }
          std::cmp::Ordering::Less => position.checked_sub(1).unwrap_or(len - 1),
          std::cmp::Ordering::Equal => continue,
        };
        encrypted_ref.swap(position, next_position);
        position = next_position;
      }
    }
  }
  encrypted_ref
    .iter()
    .cycle()
    .skip_while(|e| ***e != 0)
    .take(3003)
    .enumerate()
    .filter_map(|(i, e)| {
      // dbg!((i, e));
      match i {
        1000 | 2000 | 3000 => Some(**e),
        _ => None,
      }
    })
    .sum()
}
