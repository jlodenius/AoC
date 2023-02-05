use std::cell::RefCell;
use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete,
    combinator::map,
    multi::separated_list0,
    sequence::{preceded, separated_pair},
    IResult,
};

#[derive(Debug)]
struct Valve {
    name: String,
    flow_rate: i32,
    tunnels: Vec<String>,
    is_on: bool,
}

impl Valve {
    // Should have used nom::sequence::tuple to parse this in one go..
    // but leaving as is for now
    fn parse(input: &str) -> IResult<&str, Valve> {
        let (input, (name, flow_rate)) = separated_pair(
            preceded(tag("Valve "), complete::alpha1),
            tag(" has flow rate="),
            complete::i32,
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
                is_on: false,
            },
        ))
    }
}

// Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
fn part_one() {
    let valves: Vec<Valve> = include_str!("input.txt")
        .lines()
        .map(|line| {
            let (_, valve) = Valve::parse(line).unwrap();
            return valve;
        })
        .collect();

    let mut valve_map = HashMap::new();

    for valve in valves.iter() {
        valve_map.insert(valve.name.to_owned(), valve);
    }

    println!("{:?}", valve_map);

    let current_valve = &valves[7];

    for minute in 1..=1 {
        println!("Min {minute}");
        println!("You are at valve {}", current_valve.name);

        // 1. Check all other valves and calc the value of turning them on
        for valve in valves.iter() {
            if valve.name == current_valve.name {
                continue;
            }
            let shortest_path = shortest_path_between(current_valve, valve, &valve_map, 1, vec![]);
            println!("Shortest path = {shortest_path}");
        }
    }
}

fn shortest_path_between(
    a: &Valve,
    b: &Valve,
    valve_map: &HashMap<String, &Valve>,
    count: u32,
    mut tail: Vec<String>,
) -> u32 {
    tail.push(a.name.to_owned());

    if a.tunnels.contains(&b.name) {
        return count;
    }

    return a
        .tunnels
        .iter()
        .map(|connection_name| {
            if tail.contains(connection_name) {
                return u32::MAX; // Means we already visited this node
            }
            let connection = valve_map.get(connection_name).unwrap();
            let new_tail = tail.iter().map(|s| s.to_owned()).collect();
            return shortest_path_between(connection, b, valve_map, count + 1, new_tail);
        })
        .min()
        .unwrap();
}

fn main() {
    println!("--- PART ONE ---");
    part_one();
}
