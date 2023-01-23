use std::{
    collections::{HashMap, HashSet},
    fmt::{Display, Formatter, Result},
};

#[derive(PartialEq, Eq, Debug)]
struct Point {
    y: usize,
    x: usize,
}

#[derive(Debug)]
struct Polyline {
    line: Vec<Point>,
}

impl Polyline {
    fn get_points(&self) -> HashSet<(usize, usize)> {
        let mut points_set: HashSet<(usize, usize)> = HashSet::new();
        for window in self.line.windows(2) {
            let a = &window[0];
            let b = &window[1];

            // each point will only move in either x or y
            let move_y = a.y != b.y;

            if move_y {
                for y in a.y..=b.y {
                    points_set.insert((y, a.x));
                }
            } else {
                for x in a.x..=b.x {
                    points_set.insert((a.y, x));
                }
            }
        }
        return points_set;
    }
}

struct Grid {
    min_x: usize,
    max_x: usize,
    min_y: usize,
    max_y: usize,
    points: HashMap<(usize, usize), char>,
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut the_str = String::new();
        for y in self.min_y..=self.max_y {
            let mut line = String::new();
            for x in self.min_x..=self.max_x {
                line.push(*self.points.get(&(y, x)).unwrap());
            }
            the_str.push_str(&line);
            the_str.push('\n');
        }
        return f.write_str(&the_str);
    }
}

fn main() {
    let mut lines: Vec<_> = vec![];

    for line in include_str!("input.txt").split("\n") {
        // because last line..
        if line == "" {
            continue;
        }
        let mut polyline = Polyline { line: vec![] };
        for xy_pairs in line.split(" -> ") {
            let mut xy = xy_pairs.split(",");
            let x: usize = xy.next().unwrap().parse().unwrap();
            let y: usize = xy.next().unwrap().parse().unwrap();

            polyline.line.push(Point { y, x })
        }
        lines.push(polyline);
    }

    // Find smallest and biggest points to determine how big the grid is
    let points: Vec<&Point> = lines.iter().flat_map(|pl| &pl.line).collect();

    let mut min_y = usize::max_value();
    let mut min_x = usize::max_value();
    let mut max_y = usize::min_value();
    let mut max_x = usize::min_value();

    for point in points {
        if point.x < min_x {
            min_x = point.x
        }
        if point.y < min_y {
            min_y = point.y
        }
        if point.x > max_x {
            max_x = point.x
        }
        if point.y > max_y {
            max_y = point.y
        }
    }

    // Create and populate the grid
    let mut grid = Grid {
        min_y,
        max_y,
        min_x,
        max_x,
        points: HashMap::new(),
    };

    // Insert "air"
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            grid.points.insert((y, x), '.');
        }
    }

    // Insert "rocks"
    for pl in lines.iter() {
        let points = pl.get_points();
        for p in points.iter() {
            grid.points.insert(*p, '#');
        }
    }

    println!("{}", grid);
}
