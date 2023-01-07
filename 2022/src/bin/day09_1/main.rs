use std::collections::HashSet;
use std::fs;

struct Point {
    x: usize,
    y: usize,
    visited_coordinates: HashSet<(usize, usize)>,
}

impl Point {
    fn move_x(&mut self, steps: usize, is_left: bool) {
        let new_x = match is_left {
            true => self.x - steps,
            false => self.x + steps,
        };
        self.x = new_x;
        self.visited_coordinates.insert((new_x, self.y));
    }
    fn move_y(&mut self, steps: usize, is_down: bool) {
        let new_y = match is_down {
            true => self.y - steps,
            false => self.y + steps,
        };
        self.y = new_y;
        self.visited_coordinates.insert((self.x, new_y));
    }
    fn set_position(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
        self.visited_coordinates.insert((x, y));
    }
    fn get_adjacent_coordinates(&self) -> Vec<(usize, usize)> {
        let mut adjacent_coordinates = vec![];
        for grid_x in self.x - 1..=self.x + 1 {
            for grid_y in self.y - 1..=self.y + 1 {
                adjacent_coordinates.push((grid_x, grid_y));
            }
        }
        return adjacent_coordinates;
    }
}

fn move_one_step(direction: &str, head: &mut Point, tail: &mut Point) {
    // move head
    match direction {
        "U" => head.move_y(1, false),
        "D" => head.move_y(1, true),
        "L" => head.move_x(1, true),
        "R" => head.move_x(1, false),
        _ => {}
    }

    // check if tail is adjacent
    let head_adjacent_coordinates = head.get_adjacent_coordinates();
    let is_adjacent = head_adjacent_coordinates.iter().any(|(x, y)| {
        if tail.x == *x && tail.y == *y {
            return true;
        }
        return false;
    });

    // tail needs to move
    if !is_adjacent {
        println!("-- MOVING TAIL --");
        println!("Head is at: {}, {}", head.x, head.y);
        println!("Tail is at: {}, {}", tail.x, tail.y);

        match direction {
            "U" => tail.set_position(head.x, head.y - 1),
            "D" => tail.set_position(head.x, head.y + 1),
            "L" => tail.set_position(head.x + 1, head.y),
            "R" => tail.set_position(head.x - 1, head.y),
            _ => {}
        }

        println!("Tail moved to: {}, {}", tail.x, tail.y);
        println!("\n\n");
    }
}

fn main() {
    let file_path = "day9_input.txt";
    let contents = fs::read_to_string(file_path).unwrap();

    // starting at 1000 / 1000 just to avoid negative numbers
    // and being able to use usize
    let mut head = Point {
        x: 1000,
        y: 1000,
        visited_coordinates: HashSet::from_iter(vec![(1000, 1000)]),
    };
    let mut tail = Point {
        x: 1000,
        y: 1000,
        visited_coordinates: HashSet::from_iter(vec![(1000, 1000)]),
    };

    for line in contents.lines() {
        let command: Vec<_> = line.split_whitespace().collect();

        let (direction, steps) = match command[..] {
            [d, s] => (d, s),
            _ => return,
        };
        let steps: usize = steps.parse().unwrap();

        println!("{} - {}\n", direction, steps);

        for _ in 0..steps {
            move_one_step(direction, &mut head, &mut tail);
        }
    }

    println!(
        "Unique positions for tail: {:?}",
        tail.visited_coordinates.len()
    );
}
