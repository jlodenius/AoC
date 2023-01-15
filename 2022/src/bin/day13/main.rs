use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::{cmp::Ordering, fmt::Debug};

// Deserialize is required by serde
#[derive(PartialEq, Deserialize)]
#[serde(untagged)] // To represent this enum without tags, more info: https://serde.rs/enum-representations.html
enum Node {
    Int(u32),
    List(Vec<Node>),
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(val) => write!(f, "{val}"),
            Self::List(val) => f.debug_list().entries(val).finish(),
        }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Node::Int(a), Node::Int(b)) => a.partial_cmp(b),
            (left, right) => {
                match left {
                    Node::Int(val) => {
                        println!("in partial_cmp INT {:?}", val);
                    }
                    Node::List(val) => {
                        println!("in partial_cmp LIST {:?}", val);
                    }
                }
                return Some(Ordering::Equal);
            }
        }
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

        let test = left.partial_cmp(&right);
        println!("Test -> {:?}", test);
    }
}

fn main() {
    part_one();
}
