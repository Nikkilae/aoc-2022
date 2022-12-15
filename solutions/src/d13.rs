use std::{
    borrow::BorrowMut,
    cmp::{min, Ordering},
    error::Error,
    iter::Peekable,
    str::Chars,
};

#[derive(Clone)]
enum Packet {
    List(Vec<Packet>),
    Value(i32),
}
impl Packet {
    fn as_list(&self) -> Packet {
        match self {
            Packet::List(_) => self.clone(),
            Packet::Value(v) => Packet::List(vec![Packet::Value(*v)]),
        }
    }
}
impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(compare_packets(self, other))
    }
}
impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        compare_packets(self, other)
    }
}
impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        compare_packets(self, other) == Ordering::Equal
    }
}
impl Eq for Packet {}

fn compare_packets(p1: &Packet, p2: &Packet) -> Ordering {
    if let (Packet::Value(a), Packet::Value(b)) = (p1, p2) {
        let ord = a.cmp(b);
        if ord != Ordering::Equal {
            return ord;
        }
    } else if let (Packet::List(a_list), Packet::List(b_list)) = (p1, p2) {
        let a_len = a_list.len();
        let b_len = b_list.len();
        for i in 0..min(a_len, b_len) {
            let ord = compare_packets(&a_list[i], &b_list[i]);
            if ord != Ordering::Equal {
                return ord;
            }
        }
        let ord = a_len.cmp(&b_len);
        if ord != Ordering::Equal {
            return ord;
        }
    } else {
        let ord = compare_packets(&p1.as_list(), &p2.as_list());
        if ord != Ordering::Equal {
            return ord;
        }
    }
    Ordering::Equal
}

fn parse_array(input: &mut Peekable<&mut Chars>) -> Option<Vec<Packet>> {
    let Some(char) = input.peek().copied() else {
        return None;
    };
    if char != '[' {
        return None;
    }
    input.next();
    if input.peek() == Some(&']') {
        input.next();
        return Some(Vec::new());
    }
    let mut packets = Vec::new();
    packets.push(parse_packet(input)?);
    while input.peek() == Some(&',') {
        input.next();
        packets.push(parse_packet(input)?);
    }
    if input.peek() != Some(&']') {
        return None;
    }
    input.next();
    Some(packets)
}

fn parse_value(input: &mut Peekable<&mut Chars>) -> Option<i32> {
    let mut str = String::new();
    loop {
        let Some(char) = input.peek().copied() else {
            break;
        };
        if !char.is_numeric() {
            break;
        }
        input.next();
        str.push(char);
    }
    if str.is_empty() {
        None
    } else {
        str.parse().ok()
    }
}

fn parse_packet(input: &mut Peekable<&mut Chars>) -> Option<Packet> {
    if let Some(arr) = parse_array(input) {
        return Some(Packet::List(arr));
    } else if let Some(value) = parse_value(input) {
        return Some(Packet::Value(value));
    }
    None
}

fn parse_packet_pairs(input: &str) -> Vec<(Packet, Packet)> {
    input
        .split("\r\n\r\n")
        .filter_map(|str| {
            let mut lines = str.lines();
            let packet1 = parse_packet(&mut lines.next()?.chars().borrow_mut().peekable())?;
            let packet2 = parse_packet(&mut lines.next()?.chars().borrow_mut().peekable())?;
            Some((packet1, packet2))
        })
        .collect()
}

fn parse_packets(input: &str) -> Vec<Packet> {
    input
        .lines()
        .filter_map(|line| parse_packet(&mut line.chars().borrow_mut().peekable()))
        .collect()
}

pub fn solve_part_1(input: &str) -> Result<String, Box<dyn Error>> {
    let pairs = parse_packet_pairs(input);
    let mut n = 0;
    for (i, (p1, p2)) in pairs.iter().enumerate() {
        if p1.cmp(p2) == Ordering::Less {
            n += i + 1;
        }
    }
    Ok(n.to_string())
}

pub fn solve_part_2(input: &str) -> Result<String, Box<dyn Error>> {
    let mut packets = parse_packets(input);
    let p1 = Packet::List(vec![Packet::List(vec![Packet::Value(2)])]);
    let p2 = Packet::List(vec![Packet::List(vec![Packet::Value(6)])]);
    packets.push(p1.clone());
    packets.push(p2.clone());
    packets.sort();
    let i1 = packets.iter().position(|p| p == &p1).unwrap() + 1;
    let i2 = packets.iter().position(|p| p == &p2).unwrap() + 1;
    Ok((i1 * i2).to_string())
}
