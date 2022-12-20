use std::str::FromStr;

pub fn compute(s: &str) -> (usize, usize) {
  let blueprints = s
    .lines()
    .map(|l| l.parse::<Blueprint>().unwrap())
    .collect::<Vec<_>>();
  for blueprint in blueprints {
    println!("{blueprint:?}: {:?}", blueprint.max_geodes());
  }
  (0, 0)
}

#[derive(Debug, Copy, Clone)]
struct Blueprint {
  id: u8,
  ore_robot_ore: u8,
  clay_robot_ore: u8,
  obsidian_robot_ore: u8,
  obsidian_robot_clay: u8,
  geode_robot_ore: u8,
  geode_robot_obsidian: u8,
}

enum Robot {
  Ore,
  Clay,
  Obsidian,
  Geode,
}

impl Blueprint {
  fn max_geodes(self) -> ([u8; 4], u8) {
    let mut best_config = [0; 4];
    let mut max_geodes = 0;
    for ore_robot in 1..=3 {
      for clay_robot in 1..=4 {
        for obsidian_robot in 1..=4 {
          println!("Exploring config for {ore_robot}, {clay_robot}, {obsidian_robot}");
          let mut ore = 0;
          let mut clay = 0;
          let mut obsidian = 0;
          let mut geode = 0;
          let mut ore_robots = 1;
          let mut clay_robots = 0;
          let mut obsidian_robots = 0;
          let mut geode_robots = 0;
          for i in 1..=24 {
            macro_rules! increment {
                () => {
                  ore += ore_robots;
                  clay += clay_robots;
                  obsidian += obsidian_robots;
                  geode += geode_robots;
                  println!("{i} incrementing: {ore}, {clay}, {obsidian}, {geode} -> {ore_robots}, {clay_robots}, {obsidian_robots}, {geode_robots}")
                };
              };
            if ore_robots < ore_robot && ore >= self.ore_robot_ore {
              ore -= self.ore_robot_ore;
              increment!();
              ore_robots += 1;
            } else if clay_robots < clay_robot && ore >= self.clay_robot_ore {
              ore -= self.clay_robot_ore;
              increment!();
              clay_robots += 1;
            } else if obsidian_robots < obsidian_robot
              && clay >= self.obsidian_robot_clay
              && ore >= self.obsidian_robot_ore
            {
              clay -= self.obsidian_robot_clay;
              ore -= self.obsidian_robot_ore;
              increment!();
              obsidian_robots += 1;
            } else if obsidian >= self.geode_robot_obsidian && ore >= self.obsidian_robot_ore {
              ore -= self.geode_robot_ore;
              obsidian -= self.geode_robot_obsidian;
              increment!();
              geode_robots += 1;
            } else {
              increment!();
            }
          }
          if geode > max_geodes {
            max_geodes = geode;
            best_config = [ore_robots, clay_robots, obsidian_robots, geode_robots];
          }
        }
      }
    }
    (best_config, max_geodes)
  }
}

impl FromStr for Blueprint {
  type Err = core::convert::Infallible;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let s = s.replace(":", "");
    let v = s
      .split_whitespace()
      .filter_map(|w| w.parse::<u8>().ok())
      .collect::<Vec<_>>();
    Ok(Self {
      id: v[0],
      ore_robot_ore: v[1],
      clay_robot_ore: v[2],
      obsidian_robot_ore: v[3],
      obsidian_robot_clay: v[4],
      geode_robot_ore: v[5],
      geode_robot_obsidian: v[6],
    })
  }
}

#[test]
fn test_19() {
  let input = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.";
  dbg!(compute(input));
  let input2 = "Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";
  dbg!(compute(input2));
  panic!();
}
