use std::{collections::HashMap, ops::RangeInclusive, str::FromStr};

pub fn compute(s: &str) -> (i32, i64) {
  let sensors = s.parse::<Sensors>().unwrap();
  let mut overlaps = vec![];
  for (sensor, beacon) in sensors.sensors.iter().copied() {
    if let Some(range) = Sensors::range_at_line(2_000_000, sensor, beacon) {
      overlaps.push(range);
    }
  }
  overlaps.sort_by_key(|r| *r.start());
  let (no_beacon, _) = Sensors::coalesce_ranges(&overlaps, i32::MIN, i32::MAX);
  let mut tuning_frequency = 0;
  for line in (0..=4_000_000).rev() {
    let mut overlaps = vec![];
    for (sensor, beacon) in sensors.sensors.iter().copied() {
      if let Some(range) = Sensors::range_at_line(line, sensor, beacon) {
        overlaps.push(range);
      }
    }
    overlaps.sort_by_key(|r| *r.start());
    let (no_beacon, gap) = Sensors::coalesce_ranges(&overlaps, 0, 4_000_000);
    if no_beacon == 4_000_000 {
      tuning_frequency = gap.unwrap() as i64 * 4000000 + line as i64;
      break;
    }
  }
  (no_beacon - 1, tuning_frequency)
}

#[derive(Debug)]
struct Sensors {
  sensors: Vec<((i32, i32), (i32, i32))>,
}

impl Sensors {
  fn range_at_line(
    line: i32,
    sensor: (i32, i32),
    beacon: (i32, i32),
  ) -> Option<RangeInclusive<i32>> {
    let x_dist = (sensor.0 - beacon.0).abs();
    let y_dist = (sensor.1 - beacon.1).abs();
    let dist = x_dist + y_dist;
    let y_dist = (sensor.1 - line).abs();
    if y_dist < dist {
      Some(sensor.0 - (dist - y_dist)..=sensor.0 + (dist - y_dist))
    } else {
      None
    }
  }
  fn coalesce_ranges(ranges: &[RangeInclusive<i32>], min: i32, max: i32) -> (i32, Option<i32>) {
    let mut count = 0;
    let mut range = ranges.first().cloned().unwrap();
    let mut gap = None;
    range = min.max(*range.start())..=max.min(*range.end());
    for r in &ranges[1..] {
      if *r.end() < min || *r.start() > max || r.end() < range.end() {
        // do nothing
      } else if r.start() > range.end() {
        // not overlapping
        count += range.end() - range.start() + 1;
        if *range.end() + 1 < *r.start() {
          gap = Some(*range.end() + 1);
        }
        range = r.clone();
        range = min.max(*range.start())..=max.min(*range.end());
      } else {
        // overlapping so extend range
        // range = *range.start()..=*r.end();
        range = min.max(*range.start())..=max.min(*r.end());
      }
    }
    count += range.end() - range.start() + 1;
    (count, gap)
  }
}

impl FromStr for Sensors {
  type Err = core::convert::Infallible;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut sensors = vec![];
    for line in s.lines() {
      let line = line
        .replace("Sensor at", "")
        .replace(" x=", "")
        .replace(" y=", "")
        .replace(": closest beacon is at", ",");
      let c = line
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
      sensors.push(((c[0], c[1]), (c[2], c[3])));
    }
    Ok(Sensors { sensors })
  }
}
