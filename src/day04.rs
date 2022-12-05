use std::ops::RangeInclusive;

pub fn compute(s: &str) -> (usize, usize) {
  s.lines()
    .map(|l| {
      let mut split = l.split(',');
      (
        range_from_assignment(split.next().unwrap()),
        range_from_assignment(split.next().unwrap()),
      )
    })
    .fold((0, 0), |acc, (l, r)| {
      (
        acc.0 + usize::from(completely_overlaps(&l, &r)),
        acc.1 + usize::from(partially_overlaps(&l, &r)),
      )
    })
}

fn range_from_assignment(s: &str) -> RangeInclusive<usize> {
  let mut split = s.split('-');
  let (l, r) = (split.next().unwrap(), split.next().unwrap());
  l.parse().unwrap()..=r.parse().unwrap()
}

fn completely_overlaps(l: &RangeInclusive<usize>, r: &RangeInclusive<usize>) -> bool {
  (l.start() >= r.start() && l.end() <= r.end()) || (r.start() >= l.start() && r.end() <= l.end())
}

fn partially_overlaps(l: &RangeInclusive<usize>, r: &RangeInclusive<usize>) -> bool {
  l.start() >= r.start() && l.start() <= r.end()
    || l.end() >= r.start() && l.end() <= r.end()
    || r.start() >= l.start() && r.start() <= l.end()
    || r.end() >= l.start() && r.end() <= l.end()
}
