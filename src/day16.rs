use std::{
  collections::{BTreeMap, HashMap},
  str::FromStr,
};

pub fn compute(s: &str) -> (usize, usize) {
  let graph = s.parse::<TunnelGraph>().unwrap();
  let start = SearchKey {
    valve: graph.graph.len() as u8 - 1,
    enabled_valves: 0,
    remaining_steps: 30,
  };
  let valves_to_open = (1 << graph.flows.len()) - 1;
  // let output = pathfinding::directed::dfs::dfs(
  //   start,
  //   |k| graph.neighbors(k).map(|(k, _cost)| k),
  //   |k| k.remaining_steps == 0 || k.enabled_valves == valves_to_open,
  // );
  let output = pathfinding::directed::dijkstra::dijkstra(
    &start,
    |k| graph.neighbors(k),
    |k| k.remaining_steps == 0 || k.enabled_valves == valves_to_open,
  );
  dbg!(output);
  (0, 0)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct SearchKey {
  valve: u8,
  remaining_steps: u8,
  enabled_valves: u16,
}

#[derive(Debug)]
struct TunnelGraph {
  // outer vec is each possible starting point
  // inner vec is how long it takes to get to each key from the starting point.
  graph: Vec<Vec<u8>>,
  flows: Vec<i32>,
}

impl TunnelGraph {
  fn neighbors(&self, key: &SearchKey) -> impl Iterator<Item = (SearchKey, i32)> + '_ {
    println!("exploring: {key:?}");
    let key = *key;
    self.graph[key.valve as usize]
      .iter()
      .copied()
      .enumerate()
      .filter_map(move |(k, steps)| {
        key
          .remaining_steps
          .checked_sub(steps + 1)
          .filter(|_| key.enabled_valves & (1 << k) == 0)
          .map(|remaining_steps| {
            (
              SearchKey {
                valve: k as u8,
                enabled_valves: key.enabled_valves | (1 << k),
                remaining_steps,
              },
              -(self.flows[k as usize] * remaining_steps as i32),
            )
          })
      })
      .inspect(|(k, cost)| println!("{cost}, {k:?}"))
  }
}

impl FromStr for TunnelGraph {
  type Err = core::convert::Infallible;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut initial_graph = HashMap::new();
    for line in s.lines() {
      let line = line
        .replace("Valve", "")
        .replace("has flow rate=", "")
        .replace("; tunnels lead to valves", "")
        .replace("; tunnel leads to valve", "")
        .replace(',', "");
      let parts = line.split_whitespace().collect::<Vec<_>>();
      println!("{parts:?}");
      initial_graph.insert(
        parts[0].to_owned(),
        (
          parts[1].parse::<i32>().unwrap(),
          parts[2..].iter().map(|&s| s.to_owned()).collect::<Vec<_>>(),
        ),
      );
    }
    let keys = initial_graph
      .iter()
      .filter_map(|(k, (f, c))| (*f > 0).then(|| k.clone()))
      .collect::<Vec<_>>();
    dbg!(&keys);
    let aa = "AA".to_string();
    let graph = keys
      .iter()
      .chain([&aa].into_iter())
      .map(|start| {
        keys
          .iter()
          .map(|end| {
            let cost = pathfinding::directed::dijkstra::dijkstra(
              start,
              |n| {
                initial_graph
                  .get(n)
                  .unwrap()
                  .1
                  .iter()
                  .map(|s| (s.clone(), 1_u8))
              },
              |n| n == end,
            );
            cost.unwrap().1
          })
          .collect::<Vec<_>>()
      })
      .collect::<Vec<_>>();
    let flows = keys
      .iter()
      .map(|k| initial_graph.get(k).unwrap().0)
      .collect();
    dbg!(Ok(Self { graph, flows }))
  }
}

#[test]
fn test_16() {
  let input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";
  dbg!(compute(input));
  panic!();
}
