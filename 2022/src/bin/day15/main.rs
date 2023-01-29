use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

use nom::{
    bytes::streaming::tag,
    combinator::map,
    sequence::{preceded, separated_pair},
    IResult,
};

#[derive(Debug)]
struct InputData {
    sensor: Point,
    beacon: Point,
    manhattan_d: i64,
}

impl InputData {
    fn parse(i: &str) -> IResult<&str, InputData> {
        map(
            separated_pair(
                preceded(tag("Sensor at "), Point::parse),
                tag(": closest beacon is at "),
                Point::parse,
            ),
            |(sensor, beacon)| {
                let manhattan_d = sensor.manhattan_d(&beacon);
                InputData {
                    sensor,
                    beacon,
                    manhattan_d,
                }
            },
        )(i)
    }
}

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn parse(i: &str) -> IResult<&str, Point> {
        // Maps a function on the result of a parser
        map(
            // Combines the result of two parser between a separator
            // and returns a tuple of the two results
            separated_pair(
                // Discards the first parsers result and returns the second
                preceded(tag("x="), nom::character::complete::i64),
                tag(", "),
                preceded(tag("y="), nom::character::complete::i64),
            ),
            // The map function
            |(x, y)| Point { x, y },
        )(i)
    }

    fn manhattan_d(&self, other: &Point) -> i64 {
        return (self.x.abs_diff(other.x) + self.y.abs_diff(other.y)) as i64;
    }
}

fn part_one() {
    // Parse input
    let mut input_data: Vec<InputData> = vec![];
    for line in include_str!("input.txt").lines() {
        let line = InputData::parse(line).unwrap().1;
        input_data.push(line);
    }

    // Do calculations
    let mut unavailable: HashSet<i64> = HashSet::new();
    let y_to_search: i64 = 2_000_000;

    for input in input_data.iter() {
        // Search outwards from position (sensor.x, y_to_find)
        for x in input.sensor.x..=(input.sensor.x + input.manhattan_d) {
            let p = Point { x, y: y_to_search };
            if input.sensor.manhattan_d(&p) > input.manhattan_d {
                break;
            };

            unavailable.insert(x);
            unavailable.insert(input.sensor.x - x.abs_diff(input.sensor.x) as i64);
        }
    }
    // Remove all known beacons on the y_to_find-line
    // ...apparently only necessary for the test input but lets keep it anyway
    let beacons_at_y: Vec<_> = input_data
        .iter()
        .filter(|i| i.beacon.y == y_to_search)
        .map(|i| &i.beacon)
        .collect();
    for beacon in beacons_at_y.iter() {
        unavailable.remove(&beacon.x);
    }

    println!("Answer: {:?}", unavailable.len());
}

fn part_two() {
    // Parse input
    let mut input_data: Vec<InputData> = vec![];
    for line in include_str!("input.txt").lines() {
        let line = InputData::parse(line).unwrap().1;
        input_data.push(line);
    }

    let max: i64 = 4_000_000;
    // let max: i64 = 20;

    // Key = Y
    // Value = impossible x range (min, max)
    let mut impossible_x_ranges: HashMap<i64, Vec<(i64, i64)>> = HashMap::new();

    // Go through each Y
    for y in 0..=max {
        if y % 1_000_000 == 0 {
            println!("First loop {y}");
        }
        // Determine which X coordinates are impossible
        // and adds them to map of impossible x values
        for input in input_data.iter() {
            // No reason to continue
            if input.manhattan_d < input.sensor.y.abs_diff(y) as i64 {
                continue;
            }

            // Create ranges of impossible X values for this Y
            // Have to check recursively with large jumps here to improve performance
            // Checking each X one by one like in part_one would take forever
            let from = input.sensor.x;
            let (x_min, x_max) = get_x_min_max(from, y, &input);
            let map_entry = impossible_x_ranges.entry(y).or_insert(vec![]);
            map_entry.push((x_min, x_max));
        }
    }

    // Go through each Y and check against each range of X
    for y in 0..=max {
        if y % 1_000_000 == 0 {
            println!("Second loop {y}");
        }
        let ranges = impossible_x_ranges.get_mut(&y).unwrap();
        match find_unconnected_range(ranges) {
            Some(x) => {
                // We will only get one match, and that will be our X coordinate
                println!("X is {x}");
                let tuning_frequency = (x * 4_000_000) + y;
                println!("Tuning frequency is {}", tuning_frequency);
                break;
            }
            None => {} // This means the entire range is "connected"
        }
    }
}

fn get_x_min_max(x: i64, y: i64, input: &InputData) -> (i64, i64) {
    let p = Point { x, y };
    let current_manhattan_d = input.sensor.manhattan_d(&p);
    let delta = current_manhattan_d.abs_diff(input.manhattan_d) as i64;
    match current_manhattan_d.cmp(&input.manhattan_d) {
        Ordering::Equal => {
            let x_max = x;
            let x_min = input.sensor.x - x_max.abs_diff(input.sensor.x) as i64;
            return (x_min, x_max);
        }
        Ordering::Less => {
            return get_x_min_max(x + delta, y, input);
        }
        Ordering::Greater => {
            return get_x_min_max(x - delta, y, input);
        }
    }
}

fn find_unconnected_range(ranges: &mut Vec<(i64, i64)>) -> Option<i64> {
    ranges.sort_by_key(|r| r.0);

    let (start, rest) = ranges.split_first().unwrap();
    let mut max: i64 = start.1;

    for range in rest.iter() {
        if range.0 > max + 1 {
            return Some(max + 1);
        }
        max = std::cmp::max(max, range.1);
    }
    return None;
}

fn main() {
    println!("--- PART ONE ---");
    part_one();
    println!("--- PART TWO ---");
    part_two();
}
