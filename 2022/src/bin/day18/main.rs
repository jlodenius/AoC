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
        let mut unexposed = 0;

        // check x
        if let Some(_) = cm.get(&[self.x - 1, self.y, self.z]) {
            unexposed += 1
        }
        if let Some(_) = cm.get(&[self.x + 1, self.y, self.z]) {
            unexposed += 1
        }

        // check y
        if let Some(_) = cm.get(&[self.x, self.y - 1, self.z]) {
            unexposed += 1
        }
        if let Some(_) = cm.get(&[self.x, self.y - 1, self.z]) {
            unexposed += 1
        }

        // check z
        if let Some(_) = cm.get(&[self.x, self.y, self.z - 1]) {
            unexposed += 1
        }
        if let Some(_) = cm.get(&[self.x, self.y, self.z + 1]) {
            unexposed += 1
        }

        return 6 - unexposed;
    }

    fn exposed_sides_2(&self, cm: &HashMap<[i32; 3], Coordinate>) -> i32 {
        let search_width = 50;
        let mut exposed = 0;

        // check x
        if let None = cm.get(&[self.x - 1, self.y, self.z]) {
            // Check further
            let start = self.x - search_width;
            let end = self.x - 1;
            let is_exposed = (start..end)
                .map(|i| cm.get(&[i, self.y, self.z]))
                .all(|val| val.is_none());

            if is_exposed {
                exposed += 1
            }
        }
        if let None = cm.get(&[self.x + 1, self.y, self.z]) {
            // Check further
            let start = self.x + 1;
            let end = self.x + search_width;
            let is_exposed = (start..end)
                .map(|i| cm.get(&[i, self.y, self.z]))
                .all(|val| val.is_none());

            if is_exposed {
                exposed += 1
            }
        }

        // check y
        if let None = cm.get(&[self.x, self.y - 1, self.z]) {
            // Check further
            let start = self.y - search_width;
            let end = self.y - 1;
            let is_exposed = (start..end)
                .map(|i| cm.get(&[self.x, i, self.z]))
                .all(|val| val.is_none());

            if is_exposed {
                exposed += 1
            }
        }
        if let None = cm.get(&[self.x, self.y - 1, self.z]) {
            // Check further
            let start = self.y + 1;
            let end = self.y + search_width;
            let is_exposed = (start..end)
                .map(|i| cm.get(&[self.x, i, self.z]))
                .all(|val| val.is_none());

            if is_exposed {
                exposed += 1
            }
        }

        // check z
        if let None = cm.get(&[self.x, self.y, self.z - 1]) {
            // Check further
            let start = self.z - search_width;
            let end = self.z - 1;
            let is_exposed = (start..end)
                .map(|i| cm.get(&[self.x, self.y, i]))
                .all(|val| val.is_none());

            if is_exposed {
                exposed += 1
            }
        }
        if let None = cm.get(&[self.x, self.y, self.z + 1]) {
            // Check further
            let start = self.z + 1;
            let end = self.z + search_width;
            let is_exposed = (start..end)
                .map(|i| cm.get(&[self.x, self.y, i]))
                .all(|val| val.is_none());

            if is_exposed {
                exposed += 1
            }
        }

        return exposed;
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

    let exposed_sides: i32 = coordinates_map
        .iter()
        .map(|(_, coordinate)| coordinate.exposed_sides_2(&coordinates_map))
        .sum();

    println!("exposed_sides {exposed_sides}");
}

fn main() {
    println!("--- PART ONE ---");
    part_one();
    println!("--- PART TWO ---");
    part_two();
}
