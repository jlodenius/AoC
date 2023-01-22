use serde::Deserialize;
use std::{cmp::Ordering, fmt::Debug, fmt::Formatter};

// Deserialize is required by serde
#[derive(Eq, PartialEq, Clone, Deserialize)]
#[serde(untagged)] // To represent this enum without tags, more info: https://serde.rs/enum-representations.html
enum Node {
    Int(u32),
    List(Vec<Node>),
}

impl Node {
    fn as_list<T>(&self, one_shot: impl FnOnce(&[Node]) -> T) -> T {
        match self {
            Self::Int(val) => one_shot(&[Self::Int(*val)]),
            Self::List(val) => one_shot(&val[..]),
        }
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(val) => write!(f, "{val}"),
            Self::List(val) => f.debug_list().entries(val).finish(),
        }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Node::Int(a), Node::Int(b)) => {
                return a.partial_cmp(b);
            }
            (left, right) => {
                // Type parameter here is not necessary, it can be infered
                return Some(left.as_list::<Ordering>(|l| {
                    right.as_list::<Ordering>(|r| {
                        l.iter()
                            // Iterate over left and right (combined with zip)
                            .zip(r.iter())
                            // And compare the values...
                            .map(|(ll, rr)| ll.cmp(rr))
                            // Find first value that is not Equal
                            .find(|ord| *ord != Ordering::Equal)
                            // Or if everything equal, compare the lengths of the lists
                            .unwrap_or_else(|| l.len().cmp(&r.len()))
                    })
                }));
            }
        }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn part_one() {
    let sum: usize = include_str!("input.txt")
        .split("\n\n")
        .enumerate()
        .map(|(idx, line_pairs)| {
            let mut nodes = line_pairs
                .lines()
                .map(|line| serde_json::from_str::<Node>(line).unwrap());

            let left = nodes.next().unwrap();
            let right = nodes.next().unwrap();

            match left < right {
                true => idx + 1,
                false => 0,
            }
        })
        .sum();

    println!("result {:?}", sum);
}

fn part_two() {
    // Underscore as type means the compiler can infer the type
    let mut packets: Vec<_> = include_str!("input.txt")
        .split("\n\n")
        .flat_map(|line_pairs| {
            let mut nodes = line_pairs
                .lines()
                .map(|line| serde_json::from_str::<Node>(line).unwrap());

            let left = nodes.next().unwrap();
            let right = nodes.next().unwrap();
            return [left, right];
        })
        .collect();

    let d_1 = Node::List(vec![Node::Int(2)]);
    let d_2 = Node::List(vec![Node::Int(6)]);

    packets.push(d_1);
    packets.push(d_2);
    packets.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let dx_1 = &Node::List(vec![Node::Int(2)]);
    let dx_2 = &Node::List(vec![Node::Int(6)]);

    let mut decoder_key: [usize; 2] = [0, 0];

    for (idx, packet) in packets.iter().enumerate() {
        if packet == dx_1 {
            decoder_key[0] = idx + 1;
        }
        if packet == dx_2 {
            decoder_key[1] = idx + 1;
        }
    }

    let sum: usize = decoder_key.iter().product();
    println!("decoder key {}", sum);
}

fn main() {
    println!("\n--- part 1 ---\n");
    part_one();
    println!("\n--- part 2 ---\n");
    part_two();
}
