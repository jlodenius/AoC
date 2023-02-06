use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete,
    combinator::{all_consuming, map},
    multi::separated_list0,
    sequence::{preceded, separated_pair, tuple},
    Finish, IResult,
};

#[derive(Debug)]
struct Valve {
    name: String,
    flow_rate: u32,
    tunnels: Vec<String>,
}

impl Valve {
    // My first parser
    #![allow(dead_code)]
    fn parse_v1(input: &str) -> IResult<&str, Valve> {
        let (input, (name, flow_rate)) = separated_pair(
            preceded(tag("Valve "), complete::alpha1),
            tag(" has flow rate="),
            complete::u32,
        )(input)?;

        let (input, tunnels) = map(
            preceded(
                alt((
                    tag("; tunnels lead to valves "),
                    tag("; tunnel leads to valve "),
                )),
                separated_list0(tag(", "), complete::alpha1),
            ),
            |strings| strings.into_iter().map(String::from).collect(),
        )(input)?;

        Ok((
            input,
            Valve {
                name: name.to_owned(),
                flow_rate,
                tunnels,
            },
        ))
    }
    // Had to be improved
    fn parse_v2(input: &str) -> IResult<&str, Valve> {
        map(
            tuple((
                preceded(tag("Valve "), take(2usize)),
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
                name: String::from(a),
                flow_rate: b,
                tunnels: c.into_iter().map(String::from).collect(),
            },
        )(input)
    }
}

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
        valve_map.insert(valve.name.to_owned(), valve);
    }

    println!("{:?}", valve_map);

    // Calculations
    let mut current_valve = &valves[0];
    let mut minute: u32 = 1;
    let mut total_score = 0;
    let mut active_valves: Vec<&str> = vec![];

    // Start loop
    while minute < 30 {
        println!("Min {minute}");
        println!("You are at valve {}", current_valve.name);

        let minutes_left = (30 - minute) as u32;

        let mut current_highest_value: Option<(&Valve, u32, u32)> = None;

        // 1. Check all other valves and calc the value of turning them on
        for valve in valves.iter() {
            if active_valves.contains(&valve.name.as_str()) {
                continue;
            }
            if valve.name == current_valve.name {
                continue;
            }
            let shortest_path = shortest_path_between(current_valve, valve, &valve_map, 1, vec![]);
            let shortest_path_len = shortest_path.len() as u32;

            // No value
            if minutes_left <= shortest_path_len {
                continue;
            }

            let value = (minutes_left - shortest_path_len - 1) as u32 * valve.flow_rate;

            println!(
                "Shortest path to {} {:?} | flow_rate {}",
                valve.name, shortest_path, valve.flow_rate
            );

            match current_highest_value {
                Some((_, x_value, _)) => {
                    if value > x_value {
                        current_highest_value = Some((valve, value, shortest_path_len as u32));
                    }
                }
                None => current_highest_value = Some((valve, value, shortest_path_len as u32)),
            }
            println!("VALUE -> {}", value);
        }

        // UPDATE VALUES AND SET VALVE TO ACTIVE
        match current_highest_value {
            Some((high_valve, high_score, high_path)) => {
                active_valves.push(&high_valve.name);
                current_valve = high_valve;
                total_score += high_score;
                minute += high_path;
            }
            None => {
                // Nothing more to do
                minute = 30;
            }
        }
    }

    println!("DONE -------\nSCORE -> {}", total_score);
}

fn shortest_path_between(
    a: &Valve,
    b: &Valve,
    valve_map: &HashMap<String, &Valve>,
    count: u32,
    mut tail: Vec<String>,
) -> Vec<String> {
    tail.push(a.name.to_owned());

    if a.tunnels.contains(&b.name) {
        // Ugly copying..
        let (_, relevant_tail) = tail.split_first().unwrap();
        let mut val = Vec::from(relevant_tail);
        val.push(b.name.to_owned());
        return val;
    }

    let mut sorted = a
        .tunnels
        .iter()
        .map(|connection_name| {
            if tail.contains(connection_name) {
                // extremely shitty solution, fix later..
                // the point is to completely discard this
                // trail once we get to a Valve we already visited
                // i.e. we are in a never ending loop
                return vec![
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                ];
            }
            let connection = valve_map.get(connection_name).unwrap();
            let new_tail = tail.iter().map(|s| s.to_owned()).collect();
            return shortest_path_between(connection, b, valve_map, count + 1, new_tail);
        })
        .collect::<Vec<Vec<String>>>();
    sorted.sort_by(|a, b| a.len().cmp(&b.len()));

    // Another ugly copy, can refactor Valve.name to be a byte array
    // instead of a String to fix this issue... LATER
    return sorted[0].iter().map(String::from).collect();
}

fn main() {
    println!("--- PART ONE ---");
    part_one();
}
