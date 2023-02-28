use std::{collections::HashMap, fmt::Debug, num::ParseIntError, str::FromStr};

struct Coordinate {
    x: i32,
    y: i32,
    z: i32,
}

impl FromStr for Coordinate {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let xyz: Vec<&str> = s.split(",").collect();

        let x = xyz[0].parse()?;
        let y = xyz[1].parse()?;
        let z = xyz[2].parse()?;

        Ok(Coordinate { x, y, z })
    }
}

impl Debug for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Coordinate { x, y, z } = self;
        write!(f, "(X: {}, Y: {}, Z: {})", x, y, z)
    }
}

impl PartialEq for Coordinate {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Coordinate {
    fn exposed_sides(&self, cm: &HashMap<[i32; 3], Coordinate>) -> i32 {
        let unexposed: i32 = [-1, 1]
            .iter()
            .map(|add_to| {
                let mut val = 0;
                // check x
                if let Some(_) = cm.get(&[self.x + add_to, self.y, self.z]) {
                    val += 1
                }
                // check y
                if let Some(_) = cm.get(&[self.x, self.y + add_to, self.z]) {
                    val += 1
                }
                // check z
                if let Some(_) = cm.get(&[self.x, self.y, self.z + add_to]) {
                    val += 1
                }
                val
            })
            .sum();
        return 6 - unexposed;
    }
}

fn part_one() {
    let mut coordinates_map = HashMap::new();

    for line in include_str!("input.txt").lines() {
        let coordinate = line.parse::<Coordinate>().unwrap();
        coordinates_map.insert([coordinate.x, coordinate.y, coordinate.z], coordinate);
    }

    let exposed_sides: i32 = coordinates_map
        .iter()
        .map(|(_, coordinate)| coordinate.exposed_sides(&coordinates_map))
        .sum();

    println!("exposed_sides {exposed_sides}");
}

fn part_two() {
    let mut coordinates_map = HashMap::new();

    for line in include_str!("input.txt").lines() {
        let coordinate = line.parse::<Coordinate>().unwrap();
        coordinates_map.insert([coordinate.x, coordinate.y, coordinate.z], coordinate);
    }

    // TODO
}

fn main() {
    println!("--- PART ONE ---");
    part_one();
    println!("--- PART TWO ---");
    part_two();
}
