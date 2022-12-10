pub fn compute(s: &str) -> (isize, String) {
  let mut x = 1_isize;
  let mut cycle = 1;
  let mut signal_strengths = 0;
  let cycles_for_strengths = vec![20, 60, 100, 140, 180, 220];
  let mut crt = vec![' '; 240];
  for line in s.lines() {
    let (cycle_count, adder) = match line {
      "noop" => (1_isize, 0_isize),
      _ => (2, line[5..].parse::<isize>().unwrap()),
    };
    // println!("before: {cycle_count}, {cycle}, {adder}, {x}");

    // println!("after : {cycle_count}, {cycle}, {adder}, {x}");

    if let Some(cycle_for_strength) = cycles_for_strengths
      .iter()
      .find(|s| (cycle..cycle + cycle_count).contains(s))
    {
      signal_strengths += x * *cycle_for_strength;
    }
    for i in 0..cycle_count {
      if (x..=x + 2).contains(&((cycle + i) % 40)) {
        crt[(cycle + i) as usize - 1] = '#';
      }
    }

    cycle += cycle_count;
    x += adder;
  }
  let mut s = "".to_string();
  for line in crt.chunks(40) {
    s.extend(line);
    s.push('\n');
  }
  (signal_strengths, s)
}
