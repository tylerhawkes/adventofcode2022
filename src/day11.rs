use std::{cell::RefCell, collections::VecDeque, sync::Mutex};

pub fn compute(s: &str) -> (usize, usize) {
  let mut monkeys = vec![
    Monkey {
      items: vec![83, 97, 95, 67],
      operation: |old| old * 19,
      test_div: 17,
      true_monkey: 2,
      false_monkey: 7,
      inspections: 0,
    },
    Monkey {
      items: vec![71, 70, 79, 88, 56, 70],
      operation: |old| old + 2,
      test_div: 19,
      true_monkey: 7,
      false_monkey: 0,
      inspections: 0,
    },
    Monkey {
      items: vec![98, 51, 51, 63, 80, 85, 84, 95],
      operation: |old| old + 7,
      test_div: 7,
      true_monkey: 4,
      false_monkey: 3,
      inspections: 0,
    },
    Monkey {
      items: vec![77, 90, 82, 80, 79],
      operation: |old| old + 1,
      test_div: 11,
      true_monkey: 6,
      false_monkey: 4,
      inspections: 0,
    },
    Monkey {
      items: vec![68],
      operation: |old| old * 5,
      test_div: 13,
      true_monkey: 6,
      false_monkey: 5,
      inspections: 0,
    },
    Monkey {
      items: vec![60, 94],
      operation: |old| old + 5,
      test_div: 3,
      true_monkey: 1,
      false_monkey: 0,
      inspections: 0,
    },
    Monkey {
      items: vec![81, 51, 85],
      operation: |old| old * old,
      test_div: 5,
      true_monkey: 5,
      false_monkey: 1,
      inspections: 0,
    },
    Monkey {
      items: vec![98, 81, 63, 65, 84, 71, 84],
      operation: |old| old + 3,
      test_div: 2,
      true_monkey: 2,
      false_monkey: 3,
      inspections: 0,
    },
  ];
  (
    compute_monkey_business(monkeys.clone(), 20, 3),
    compute_monkey_business(monkeys, 10000, 1),
  )
}

fn compute_monkey_business(monkeys: Vec<Monkey>, iterations: usize, divisor: usize) -> usize {
  let monkeys = monkeys.into_iter().map(RefCell::new).collect::<Vec<_>>();
  let lcm = monkeys
    .iter()
    .map(|m| m.borrow().test_div)
    .product::<usize>();
  for _ in 0..iterations {
    for monkey_id in 0..monkeys.len() {
      let mut current_monkey = monkeys[monkey_id].borrow_mut();
      for item in std::mem::take(&mut current_monkey.items) {
        current_monkey.inspections += 1;
        let new_worry = (current_monkey.operation)(item);
        let worry = (new_worry % lcm) / divisor;
        if worry % current_monkey.test_div == 0 {
          monkeys[current_monkey.true_monkey]
            .borrow_mut()
            .items
            .push(worry);
        } else {
          monkeys[current_monkey.false_monkey]
            .borrow_mut()
            .items
            .push(worry);
        }
      }
    }
  }
  let mut inspections = monkeys
    .iter()
    .map(|m| m.borrow().inspections)
    .collect::<Vec<_>>();
  inspections.sort_by_key(|i| std::cmp::Reverse(*i));
  inspections[0] * inspections[1]
}

#[derive(Clone)]
struct Monkey {
  items: Vec<Worry>,
  operation: fn(Worry) -> Worry,
  test_div: usize,
  true_monkey: usize,
  false_monkey: usize,
  inspections: usize,
}

type Worry = usize;
