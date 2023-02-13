use std::{cmp, collections::HashMap, fmt::Debug, hash::Hash, time::Instant};

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
    flow_rate: i32,
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
                preceded(tag(" has flow rate="), complete::i32),
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

    // Put valves in HashMap for easy access
    let mut valve_map = HashMap::new();
    for valve in valves.iter() {
        valve_map.insert(valve.name, valve);
    }

    // Starting state
    let score = 0;
    let minutes_left = 30;
    let closed_valves: Vec<ValveName> = valves.iter().map(|v| v.name).collect();
    println!("closed valves {closed_valves:?}");

    // Starting node
    let a = valve_map.get(&ValveName(['A' as u8, 'A' as u8])).unwrap();

    // Brute force each path to find best score
    let start = Instant::now();
    let max_value = brute_force(a, closed_valves, &valve_map, minutes_left, score);
    let duration = start.elapsed();

    println!("max value is = {max_value}");
    println!("time elapsed = {duration:?}");
}

fn calc_total_score(steps: i32, minutes_left: i32, flow_rate: i32) -> i32 {
    let minutes_open = minutes_left - steps - 1;
    let score = minutes_open * flow_rate;
    return cmp::max(score, 0);
}

fn brute_force(
    current_valve: &Valve,
    closed_valves: Vec<ValveName>,
    valve_map: &HashMap<ValveName, &Valve>,
    minutes_left: i32,
    score: i32,
) -> i32 {
    // 1. Find the shortest path to each closed valve (filtering out current valve)
    closed_valves
        .iter()
        // Dont check any valves with 0 flow rate
        // Or the one we are currently at
        .filter(|v| {
            let b = valve_map.get(v).unwrap();
            match b.flow_rate {
                0 => false,
                _ => *v != &current_valve.name,
            }
        })
        .map(|closed_valve| {
            let b = valve_map.get(closed_valve).unwrap();
            let shortest_path = get_shortest_path(current_valve, b, valve_map, vec![], 1);
            // println!("shortest path to {:?} = {shortest_path}", b.name);
            let total_score = calc_total_score(shortest_path, minutes_left, b.flow_rate);
            // println!("opening would yield {total_score}");

            let mut new_closed_valves = closed_valves.clone();
            new_closed_valves.retain(|v| v != closed_valve);
            return brute_force(
                b,
                new_closed_valves,
                valve_map,
                minutes_left - shortest_path - 1,
                score + total_score,
            );
        })
        .max()
        .unwrap_or(score)
}

fn get_shortest_path(
    a: &Valve,
    b: &Valve,
    valve_map: &HashMap<ValveName, &Valve>,
    visited_nodes: Vec<ValveName>,
    count: i32,
) -> i32 {
    a.tunnels
        .iter()
        .map(|tunnel| {
            if visited_nodes.contains(tunnel) {
                // When we encounter a loop (I.E AA -> BB -> AA)
                // return i32 max to ignore this path
                return i32::MAX;
            }
            if tunnel == &b.name {
                return count;
            }

            // Unfortunately have to copy visited_nodes here
            // in order to pass it to the next recursion
            let mut new_visited_nodes = visited_nodes.clone();
            new_visited_nodes.push(a.name);

            let next = valve_map.get(&tunnel).unwrap();
            return get_shortest_path(next, b, valve_map, new_visited_nodes, count + 1);
        })
        .min()
        .unwrap()
}

fn main() {
    println!("--- PART ONE ---");
    part_one();
}
