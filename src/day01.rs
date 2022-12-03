pub fn compute(input: &str) -> (usize, usize) {
  let mut calories = calories(input);
  calories.sort_by_key(|u| std::cmp::Reverse(*u));

  (calories[0], calories[..3].iter().copied().sum())
}

fn calories(input: &str) -> Vec<usize> {
  let mut result = Vec::with_capacity(10000);
  for lines in input.split("\n\n") {
    let mut val = 0;
    for line in lines.lines() {
      val += line.parse::<usize>().unwrap();
    }
    result.push(val);
  }
  result
}
