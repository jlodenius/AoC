use std::fs;

fn is_visible(y_pos: usize, x_pos: usize, grid: &Vec<Vec<u8>>) -> bool {
    let row = &grid[y_pos];
    let (left, right_including_current) = row.split_at(x_pos);
    let (current_tree, right) = right_including_current.split_first().unwrap();

    // check left
    if left.iter().all(|tree_left| tree_left < current_tree) {
        return true;
    }

    // check right
    if right.iter().all(|tree_right| tree_right < current_tree) {
        return true;
    }

    // check above
    let mut above: Vec<u8> = Vec::new();
    for y in 0..y_pos {
        let tree_above = &grid[y][x_pos];
        above.push(*tree_above);
    }
    if above.iter().all(|tree_above| tree_above < current_tree) {
        return true;
    }

    // check below
    let mut below: Vec<u8> = Vec::new();
    for y in y_pos + 1..grid.len() {
        let tree_below = &grid[y][x_pos];
        below.push(*tree_below);
    }
    if below.iter().all(|tree_below| tree_below < current_tree) {
        return true;
    }

    false
}

fn main() {
    let file_path = "day8_input.txt";
    let contents = fs::read_to_string(file_path).unwrap();

    let mut grid: Vec<Vec<u8>> = Vec::new();

    for (y_pos, line) in contents.lines().enumerate() {
        for (x_pos, tree_height) in line.bytes().enumerate() {
            if x_pos == 0 {
                grid.push(vec![]);
            }
            grid[y_pos].push(tree_height);
        }
    }

    let mut visible_trees = 0;
    for (y_pos, row) in grid.iter().enumerate() {
        for (x_pos, _current_tree) in row.iter().enumerate() {
            if is_visible(y_pos, x_pos, &grid) {
                visible_trees += 1;
            }
        }
    }

    println!("Visible trees {}", visible_trees);
}
