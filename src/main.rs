#![allow(unused)]
use std::fmt::Display;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let args = std::env::args().skip(1).collect::<Vec<_>>();
  if args.is_empty() {
    return Err("pass at least one day".into());
  }
  let days: Vec<usize> = if args[0] == "all" {
    (1..=25).collect()
  } else {
    args
      .into_iter()
      .map(|a| a.parse())
      .collect::<Result<Vec<_>, _>>()?
  };
  for day in days {
    match day {
      1 => print_result(day01::compute(include_str!("../input/day01.txt"))),
      2 => print_result(day02::compute(include_str!("../input/day02.txt"))),
      3 => print_result(day03::compute(include_str!("../input/day03.txt"))),
      4 => print_result(day04::compute(include_str!("../input/day04.txt"))),
      5 => print_result(day05::compute(include_str!("../input/day05.txt"))),
      6 => print_result(day06::compute(include_str!("../input/day06.txt"))),
      7 => print_result(day07::compute(include_str!("../input/day07.txt"))),
      8 => print_result(day08::compute(include_str!("../input/day08.txt"))),
      9 => print_result(day09::compute(include_str!("../input/day09.txt"))),
      10 => print_result(day10::compute(include_str!("../input/day10.txt"))),
      11 => print_result(day11::compute(include_str!("../input/day11.txt"))),
      12 => print_result(day12::compute(include_str!("../input/day12.txt"))),
      13 => print_result(day13::compute(include_str!("../input/day13.txt"))),
      14 => print_result(day14::compute(include_str!("../input/day14.txt"))),
      15 => print_result(day15::compute(include_str!("../input/day15.txt"))),
      16 => print_result(day16::compute(include_str!("../input/day16.txt"))),
      17 => print_result(day17::compute(include_str!("../input/day17.txt"))),
      18 => print_result(day18::compute(include_str!("../input/day18.txt"))),
      19 => print_result(day19::compute(include_str!("../input/day19.txt"))),
      20 => print_result(day20::compute(include_str!("../input/day20.txt"))),
      21 => print_result(day21::compute(include_str!("../input/day21.txt"))),
      22 => print_result(day22::compute(include_str!("../input/day22.txt"))),
      23 => print_result(day23::compute(include_str!("../input/day23.txt"))),
      24 => print_result(day24::compute(include_str!("../input/day24.txt"))),
      25 => print_result(day25::compute(include_str!("../input/day25.txt"))),
      _ => return Err("Invalid day".into()),
    }
  }
  Ok(())
}

fn print_result(r: (impl Display, impl Display)) {
  println!("{}, {}", r.0, r.1);
}
