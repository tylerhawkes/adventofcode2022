use std::time::Instant;

// pub fn compute(s: &str) -> (usize, usize) {
//   let start = Instant::now();
//   let score1 = scores3(s);
//   let score2 = scores4(s);
//   let start2 = Instant::now();
//   let score = scores(s);
//   let start3 = Instant::now();
//   let score3 = (scores1(s), scores2(s));
//   let stop = Instant::now();
//   println!(
//     "{:?}, {:?}, {:?}",
//     start2 - start,
//     start3 - start2,
//     stop - start3
//   );
//   println!("({score1}, {score2}), {score:?}, {score3:?}");
//   score
//   // (12679, 14470)
// }

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
enum Outcome {
  Win,
  Lose,
  Draw,
}

impl Outcome {
  fn points(self) -> usize {
    match self {
      Self::Win => 6,
      Self::Draw => 3,
      Self::Lose => 0,
    }
  }
}

impl From<u8> for Outcome {
  fn from(b: u8) -> Self {
    match b {
      b'X' => Self::Lose,
      b'Y' => Self::Draw,
      b'Z' => Self::Win,
      _ => unreachable!(),
    }
  }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
enum RockPaperScissors {
  Rock,
  Paper,
  Scissors,
}

impl RockPaperScissors {
  const fn play_to_win(self) -> Self {
    self.next()
  }
  const fn play_to_lose(self) -> Self {
    self.next().next()
  }
  const fn next(self) -> Self {
    match self {
      Self::Rock => Self::Paper,
      Self::Paper => Self::Scissors,
      Self::Scissors => Self::Rock,
    }
  }
  fn outcome(self, other: Self) -> Outcome {
    if self == other {
      Outcome::Draw
    } else if other == self.next() {
      Outcome::Lose
    } else if other == self.next().next() {
      Outcome::Win
    } else {
      unreachable!()
    }
  }
  fn choose(self, outcome: Outcome) -> Self {
    match outcome {
      Outcome::Draw => self,
      Outcome::Win => self.play_to_win(),
      Outcome::Lose => self.play_to_lose(),
    }
  }
  const fn points(self) -> usize {
    match self {
      Self::Rock => 1,
      Self::Paper => 2,
      Self::Scissors => 3,
    }
  }
}

impl From<u8> for RockPaperScissors {
  fn from(b: u8) -> Self {
    match b {
      b'A' | b'X' => Self::Rock,
      b'B' | b'Y' => Self::Paper,
      b'C' | b'Z' => Self::Scissors,
      _ => unreachable!(),
    }
  }
}

pub fn compute(s: &str) -> (usize, usize) {
  let mut score1 = 0;
  let mut score2 = 0;
  for line in s.lines() {
    let chars = line.as_bytes();
    let opponent = RockPaperScissors::from(chars[0]);
    let choose = RockPaperScissors::from(chars[2]);
    let outcome = choose.outcome(opponent);
    score1 += outcome.points() + choose.points();
    let outcome = Outcome::from(chars[2]);
    let choose = opponent.choose(outcome);
    score2 += outcome.points() + choose.points();
  }
  (score1, score2)
}

fn scores3(s: &str) -> usize {
  let mut score = 0;
  for line in s.lines() {
    let chars = line.as_bytes();
    let opponent = RockPaperScissors::from(chars[0]);
    let choose = RockPaperScissors::from(chars[2]);
    let outcome = choose.outcome(opponent);
    score += outcome.points() + choose.points();
  }
  score
}

fn scores4(s: &str) -> usize {
  let mut score = 0;
  for line in s.lines() {
    let chars = line.as_bytes();
    let opponent = RockPaperScissors::from(chars[0]);
    let outcome = Outcome::from(chars[2]);
    let choose = opponent.choose(outcome);
    score += outcome.points() + choose.points();
  }
  score
}

fn scores1(s: &str) -> usize {
  let mut score = 0;
  for line in s.lines() {
    let chars = line.as_bytes();
    let opponent = chars[0];
    let choose = chars[2];
    let win = if choose - b'X' == opponent - b'A' {
      3
    } else if (choose == b'X' && opponent == b'B')
      || (choose == b'Y' && opponent == b'C')
      || (choose == b'Z' && opponent == b'A')
    {
      0
    } else {
      6
    };
    score += win + (choose - b'W') as usize;
  }
  score
}

fn scores2(s: &str) -> usize {
  let mut score = 0;
  for line in s.lines() {
    let chars = line.as_bytes();
    let opponent = chars[0];
    let outcome = chars[2];
    let win = outcome - b'X';
    let choose = if win == 1 {
      opponent - b'A' + b'X'
    } else if win == 2 {
      match opponent {
        b'A' => b'Y',
        b'B' => b'Z',
        b'C' => b'X',
        _ => unreachable!(),
      }
    } else {
      match opponent {
        b'A' => b'Z',
        b'B' => b'X',
        b'C' => b'Y',
        _ => unreachable!(),
      }
    };
    score += (win as usize * 3) + (choose - b'W') as usize;
  }
  score
}
