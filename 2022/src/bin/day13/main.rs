use serde::{Deserialize, Serialize};
use serde_json::Result;
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
                println!("comparing {} to {}", a, b);
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
                            .find(|&ord| ord != Ordering::Equal)
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
    for (i, line_pairs) in include_str!("input.txt").split("\n\n").enumerate() {
        let mut nodes = line_pairs
            .lines()
            .map(|line| serde_json::from_str::<Node>(line).unwrap());

        let left = nodes.next().unwrap();
        let right = nodes.next().unwrap();

        println!("Left {:?} Right {:?}", left, right);

        let test = left > right;
        println!("left > right = {:?}", test);
    }
}

fn main() {
    part_one();
}
