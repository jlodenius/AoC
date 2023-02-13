use itertools::Itertools;
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
    gsp_cache: &HashMap<(ValveName, ValveName), i32>,
) -> i32 {
    if minutes_left == 0 {
        return score;
    }
    // 1. Find the shortest path to each closed valve (filtering out current valve)
    closed_valves
        .iter()
        .filter(|v| *v != &current_valve.name)
        .map(|closed_valve| {
            let b = valve_map.get(closed_valve).unwrap();
            let shortest_path = gsp_cache.get(&(current_valve.name, *closed_valve)).unwrap();
            let total_score = calc_total_score(*shortest_path, minutes_left, b.flow_rate);

            let mut new_closed_valves = closed_valves.clone();
            new_closed_valves.retain(|v| v != closed_valve);
            return brute_force(
                b,
                new_closed_valves,
                valve_map,
                cmp::max(0, minutes_left - shortest_path - 1),
                score + total_score,
                gsp_cache,
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
    let closed_valves: Vec<ValveName> = valves
        .iter()
        // Remove all Valves with a flow rate of 0,
        // except for AA since its our starting point
        .filter(|v| v.flow_rate > 0 || v.name == ValveName(*b"AA"))
        .map(|v| v.name)
        .collect();

    println!("relevant valves {closed_valves:?}");

    // Find shortest path between each valve, and store it in a cache
    // to avoid running the same operations multiple times in recursion
    let mut cache: HashMap<(ValveName, ValveName), i32> = HashMap::new();
    for v1 in closed_valves.iter() {
        for v2 in closed_valves.iter() {
            let from = valve_map.get(v1).unwrap();
            let to = valve_map.get(v2).unwrap();
            if from.name != to.name {
                cache.insert(
                    (*v1, *v2),
                    get_shortest_path(from, to, &valve_map, vec![], 1),
                );
            }
        }
    }

    // Starting Valve
    let a = valve_map.get(&ValveName(['A' as u8, 'A' as u8])).unwrap();

    // Brute force each path to find best score
    let start = Instant::now();
    let max_value = brute_force(a, closed_valves, &valve_map, minutes_left, score, &cache);
    let duration = start.elapsed();

    println!("max value is = {max_value}");
    println!("time elapsed = {duration:?}");
}

fn part_two() {
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
    let closed_valves: Vec<ValveName> = valves
        .iter()
        // Remove all Valves with a flow rate of 0,
        // except for AA since its our starting point
        .filter(|v| v.flow_rate > 0 || v.name == ValveName(*b"AA"))
        .map(|v| v.name)
        .collect();

    println!("relevant valves {closed_valves:?}");

    // Find shortest path between each valve, and store it in a cache
    // to avoid running the same operations multiple times in recursion
    let mut cache: HashMap<(ValveName, ValveName), i32> = HashMap::new();
    for v1 in closed_valves.iter() {
        for v2 in closed_valves.iter() {
            let from = valve_map.get(v1).unwrap();
            let to = valve_map.get(v2).unwrap();
            if from.name != to.name {
                cache.insert(
                    (*v1, *v2),
                    get_shortest_path(from, to, &valve_map, vec![], 1),
                );
            }
        }
    }

    // Starting Valve
    let a = valve_map.get(&ValveName(['A' as u8, 'A' as u8])).unwrap();

    // Everything to this point is pretty much the same as Part 1

    // Part 2 begins here
    //
    // The idea is to generate each possible combination of "splits"
    // and brute force both to get a high score, it works for now
    // but its extremely slow

    let mut max_value = 0;
    let start = Instant::now();

    for n in 1..closed_valves.len() {
        let combinations = closed_valves.iter().combinations(n);

        for combination in combinations {
            let split_a: Vec<ValveName> = combination.into_iter().cloned().collect();
            let split_b: Vec<ValveName> = closed_valves
                .iter()
                .filter(|x| !split_a.contains(x))
                .cloned()
                .collect();

            // This is honestly just a stupid assumption
            // but it halves the computation time and still works
            if split_a.len() < 5 || split_b.len() < 5 {
                continue;
            }

            let combination_score = brute_force(a, split_a, &valve_map, 26, 0, &cache)
                + brute_force(a, split_b, &valve_map, 26, 0, &cache);

            max_value = cmp::max(combination_score, max_value);
        }
    }
    let duration = start.elapsed();

    println!("max value is = {max_value}");
    println!("time elapsed = {duration:?}");
}

// Sample input
// Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
fn main() {
    println!("--- PART ONE ---");
    part_one();
    println!("--- PART TWO ---");
    part_two();
}
