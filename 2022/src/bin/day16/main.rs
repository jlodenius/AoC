#![recursion_limit = "1000"]
use std::{collections::HashMap, fmt::Debug};

use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete,
    combinator::{all_consuming, map},
    multi::separated_list0,
    sequence::{preceded, tuple},
    Finish, IResult,
};

#[derive(Debug)]
struct Valve {
    name: ValveName,
    flow_rate: u32,
    tunnels: Vec<ValveName>,
}

#[derive(Clone, Hash, Eq, PartialEq, Copy)]
struct ValveName([u8; 2]); // Valve names are always 2 uppercase letters

impl Debug for ValveName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let [a, b] = self.0;
        write!(f, "{}{}", a as char, b as char)
    }
}

impl Valve {
    // Made another (shitty) parser before, hence v2
    fn parse_v2(input: &str) -> IResult<&str, Valve> {
        map(
            tuple((
                preceded(
                    tag("Valve "),
                    map(take(2usize), |slice: &str| {
                        slice.as_bytes().try_into().unwrap()
                    }),
                ),
                preceded(tag(" has flow rate="), complete::u32),
                preceded(
                    alt((
                        tag("; tunnels lead to valves "),
                        tag("; tunnel leads to valve "),
                    )),
                    separated_list0(tag(", "), complete::alpha1),
                ),
            )),
            |(a, b, c)| Valve {
                name: ValveName(a),
                flow_rate: b,
                tunnels: c
                    .into_iter()
                    .map(|slice: &str| ValveName(slice.as_bytes().try_into().unwrap()))
                    .collect(),
            },
        )(input)
    }
}

// Sample input
// Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
fn part_one() {
    // Parsing
    let valves: Vec<Valve> = include_str!("input.txt")
        .lines()
        .map(|line| {
            let valve = all_consuming(Valve::parse_v2)(line).finish().unwrap().1;
            return valve;
        })
        .collect();
    let mut valve_map = HashMap::new();
    for valve in valves.iter() {
        valve_map.insert(valve.name, valve);
    }
    println!("{:?}", valve_map);

    // Starting state
    let minutes_left = 30;
    let open_valves: Vec<ValveName> = vec![];
}

fn main() {
    println!("--- PART ONE ---");
    part_one();
}
