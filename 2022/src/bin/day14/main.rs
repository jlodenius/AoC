use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
    fmt::{Display, Formatter, Result},
};

#[derive(PartialEq, Eq, Debug)]
struct Point {
    x: usize,
    y: usize,
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
            let move_x = a.x != b.x;

            if move_x {
                for x in min(a.x, b.x)..=max(a.x, b.x) {
                    points_set.insert((x, a.y));
                }
            } else {
                for y in min(a.y, b.y)..=max(a.y, b.y) {
                    points_set.insert((a.x, y));
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
    starting_pos: (usize, usize),
    points: HashMap<(usize, usize), char>,
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut the_str = String::new();
        for y in self.min_y..=self.max_y {
            let mut line = String::new();
            for x in self.min_x..=self.max_x {
                line.push(*self.points.get(&(x, y)).unwrap());
            }
            the_str.push_str(&line);
            the_str.push('\n');
        }
        return f.write_str(&the_str);
    }
}

impl Grid {
    fn get_next_pos(&mut self, pos: (usize, usize)) -> Option<(usize, usize)> {
        let one = (pos.0, pos.1 + 1); // below
        let two = (pos.0 - 1, pos.1 + 1); // left
        let thr = (pos.0 + 1, pos.1 + 1); // right

        for p in [one, two, thr] {
            match self.points.get(&p) {
                Some(val) => {
                    if val == &'.' {
                        return self.get_next_pos(p);
                    }
                }
                None => return None, // this means we are outside of the grid
            };
        }
        self.points.insert(pos, '+');
        Some(pos)
    }
}

fn main() {
    let mut lines: Vec<_> = vec![];

    for line in include_str!("input.txt").split("\n") {
        // because last line..
        if line == "" {
            break;
        }
        let mut polyline = Polyline { line: vec![] };
        for xy_pairs in line.split(" -> ") {
            let mut xy = xy_pairs.split(",");
            let x: usize = xy.next().unwrap().parse().unwrap();
            let y: usize = xy.next().unwrap().parse().unwrap();

            polyline.line.push(Point { x, y })
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
        min_y: 0,         // lets set to 0
        max_y: max_y + 5, // and expand bottom
        min_x: min_x - 5, // and increase width
        max_x: max_x + 5, // and increase width
        starting_pos: (500, 0),
        points: HashMap::new(),
    };

    // Insert "air"
    for y in grid.min_y..=grid.max_y {
        for x in grid.min_x..=grid.max_x {
            grid.points.insert((x, y), '.');
        }
    }

    // Insert "rocks"
    for pl in lines.iter() {
        let points = pl.get_points();
        for p in points.iter() {
            grid.points.insert(*p, '#');
        }
    }

    // Insert "sand"
    let mut sand = 0;
    let mut pos = grid.get_next_pos(grid.starting_pos);
    while pos != Option::None {
        sand += 1;
        pos = grid.get_next_pos(grid.starting_pos);
    }

    println!("{}", grid);
    println!("Amount of sand {}", sand);
}
