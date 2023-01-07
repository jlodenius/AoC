use std::collections::HashSet;
use std::fs;

struct Point {
    x: i32,
    y: i32,
    visited_coordinates: HashSet<(i32, i32)>,
}

impl Point {
    fn move_x(&mut self, steps: i32, is_left: bool) {
        let new_x = match is_left {
            true => self.x - steps,
            false => self.x + steps,
        };
        self.set_position(new_x, self.y);
    }
    fn move_y(&mut self, steps: i32, is_down: bool) {
        let new_y = match is_down {
            true => self.y - steps,
            false => self.y + steps,
        };
        self.set_position(self.x, new_y);
    }
    fn set_position(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
        self.visited_coordinates.insert((x, y));
    }
    fn get_adjacent_coordinates(&self) -> Vec<(i32, i32)> {
        let mut adjacent_coordinates = vec![];
        for grid_x in self.x - 1..=self.x + 1 {
            for grid_y in self.y - 1..=self.y + 1 {
                adjacent_coordinates.push((grid_x, grid_y));
            }
        }
        return adjacent_coordinates;
    }
}

fn diff(head_pos: (i32, i32), tail_pos: (i32, i32)) -> (i32, i32) {
    let (head_x, head_y) = head_pos;
    let (tail_x, tail_y) = tail_pos;
    return (head_x - tail_x, head_y - tail_y);
}

fn follow_with_tail(direction: &str, head: &Point, tail: &mut [Point]) {
    // next_head is the point we are currently moving
    let (next_head, next_tail) = tail.split_first_mut().unwrap();

    // check if next_head is adjacent
    let head_adjacent_coordinates = head.get_adjacent_coordinates();
    let is_adjacent = head_adjacent_coordinates.iter().any(|(x, y)| {
        if next_head.x == *x && next_head.y == *y {
            return true;
        }
        return false;
    });

    // next_head needs to move
    if !is_adjacent {
        let (dx, dy) = match diff((head.x, head.y), (next_head.x, next_head.y)) {
            // Move up/left/down/right
            (0, 2) => (0, 1),
            (0, -2) => (0, -1),
            (2, 0) => (1, 0),
            (-2, 0) => (-1, 0),
            // Move right diagonally
            (2, 1) => (1, 1),
            (2, -1) => (1, -1),
            // Move left diagonally
            (-2, 1) => (-1, 1),
            (-2, -1) => (-1, -1),
            // Move up/down diagonally
            (1, 2) => (1, 1),
            (-1, 2) => (-1, 1),
            (1, -2) => (1, -1),
            (-1, -2) => (-1, -1),
            // Move diagonally
            (-2, -2) => (-1, -1),
            (-2, 2) => (-1, 1),
            (2, -2) => (1, -1),
            (2, 2) => (1, 1),
            _ => panic!("fail"),
        };
        next_head.set_position(next_head.x + dx, next_head.y + dy);

        // if anything left of the tail, repeat
        if next_tail.len() > 0 {
            follow_with_tail(direction, next_head, next_tail);
        }
    }
}

fn move_one_step(direction: &str, head: &mut Point, tail: &mut [Point]) {
    // move head
    match direction {
        "U" => head.move_y(1, false),
        "D" => head.move_y(1, true),
        "L" => head.move_x(1, true),
        "R" => head.move_x(1, false),
        _ => {}
    }
    follow_with_tail(direction, head, tail);
}

fn generate_starting_point() -> Point {
    let start_position = 1000;
    let head = Point {
        x: start_position,
        y: start_position,
        visited_coordinates: HashSet::from_iter(vec![(start_position, start_position)]),
    };
    return head;
}

fn main() {
    let file_path = "day9_input.txt";
    let contents = fs::read_to_string(file_path).unwrap();

    let mut points = Vec::new();
    for _ in 0..10 {
        points.push(generate_starting_point());
    }

    for line in contents.lines() {
        let command: Vec<_> = line.split_whitespace().collect();

        let (direction, steps) = match command[..] {
            [d, s] => (d, s),
            _ => return,
        };
        let steps: i32 = steps.parse().unwrap();

        println!("{} - {}\n", direction, steps);

        let (head, tail) = points.split_first_mut().unwrap();
        for _ in 0..steps {
            move_one_step(direction, head, tail);
        }
    }

    let tail = points.pop().unwrap();
    println!("{:?}", tail.visited_coordinates.len());
}
