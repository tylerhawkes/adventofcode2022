use std::cmp::Ordering;

pub fn compute(s: &str) -> (usize, usize) {
  let packets = parse_input(s);
  let sum_indices = packets
    .iter()
    .enumerate()
    .filter_map(|(i, (l, r))| l.properly_sorted(r).is_le().then_some(i + 1))
    .sum::<usize>();
  let mut all_packets = packets
    .into_iter()
    .flat_map(|(l, r)| [l, r])
    .collect::<Vec<_>>();
  let divider_1 = Packet {
    inner: vec![PacketData::List(Packet {
      inner: vec![PacketData::Num(2)],
    })],
  };
  let divider_2 = Packet {
    inner: vec![PacketData::List(Packet {
      inner: vec![PacketData::Num(6)],
    })],
  };
  all_packets.push(divider_1.clone());
  all_packets.push(divider_2.clone());
  all_packets.sort_by(|l, r| l.properly_sorted(r));
  let pos_1 = all_packets.iter().position(|p| p == &divider_1).unwrap() + 1;
  let pos_2 = all_packets.iter().position(|p| p == &divider_2).unwrap() + 1;
  (sum_indices, pos_1 * pos_2)
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(u8)]
enum PacketData {
  List(Packet),
  Num(u8),
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Packet {
  inner: Vec<PacketData>,
}

impl Packet {
  fn properly_sorted(&self, other: &Self) -> Ordering {
    for (ld, rd) in self.inner.iter().zip(other.inner.iter()) {
      let ordering = match (ld, rd) {
        (PacketData::Num(ln), PacketData::Num(rn)) => ln.cmp(rn),
        (PacketData::List(ll), PacketData::List(rl)) => ll.properly_sorted(rl),
        (PacketData::Num(ln), PacketData::List(rl)) => Packet {
          inner: vec![PacketData::Num(*ln)],
        }
        .properly_sorted(rl),
        (PacketData::List(ll), PacketData::Num(rn)) => ll.properly_sorted(&Packet {
          inner: vec![PacketData::Num(*rn)],
        }),
      };
      match ordering {
        Ordering::Equal => {}
        o => return o,
      }
    }
    self.inner.len().cmp(&other.inner.len())
  }
}

fn parse_packet(chars: &mut dyn Iterator<Item = char>) -> Packet {
  let mut packet = Vec::new();
  while let Some(next) = chars.next() {
    match next {
      ',' => {}
      '[' => packet.push(PacketData::List(parse_packet(chars))),
      '0'..='9' => packet.push(PacketData::Num(next as u8 - b'0')),
      'a' => packet.push(PacketData::Num(10)),
      ']' => return Packet { inner: packet },
      _ => unreachable!(),
    }
  }
  unreachable!()
}

fn parse_input(s: &str) -> Vec<(Packet, Packet)> {
  let mut packet_pairs = vec![];
  let input = s.replace("10", "a");
  for pair in input.split("\n\n") {
    let mut p = vec![];
    for line in pair.lines() {
      let mut i = line.chars();
      assert_eq!(Some('['), i.next());
      p.push(parse_packet(&mut i));
    }
    assert_eq!(p.len(), 2);
    let p1 = p.pop().unwrap();
    let p0 = p.pop().unwrap();
    packet_pairs.push((p0, p1));
  }
  packet_pairs
}
