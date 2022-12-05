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
  for chars in s.as_bytes().chunks_exact(4) {
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
