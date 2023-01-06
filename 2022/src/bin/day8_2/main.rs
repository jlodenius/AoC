use std::fs;

fn check_direction(current_tree: &u8, path: &[u8]) -> u32 {
    let mut score = 0;
    for tree in path.iter() {
        score += 1;
        if tree >= current_tree {
            break;
        }
    }
    return score;
}

fn check_direction_rev(current_tree: &u8, path: &[u8]) -> u32 {
    let mut score = 0;
    for tree in path.iter().rev() {
        score += 1;
        if tree >= current_tree {
            break;
        }
    }
    return score;
}

fn scenic_score(y_pos: usize, x_pos: usize, grid: &Vec<Vec<u8>>) -> u32 {
    let row = &grid[y_pos];
    let (left, right_including_current) = row.split_at(x_pos);
    let (current_tree, right) = right_including_current.split_first().unwrap();

    // check left
    let score_left = check_direction_rev(current_tree, left);

    // check right
    let score_right = check_direction(current_tree, right);

    // check above
    let mut above: Vec<u8> = Vec::new();
    for y in 0..y_pos {
        let tree_above = &grid[y][x_pos];
        above.push(*tree_above);
    }
    let score_above = check_direction_rev(current_tree, &above);

    // check below
    let mut below: Vec<u8> = Vec::new();
    for y in y_pos + 1..grid.len() {
        let tree_below = &grid[y][x_pos];
        below.push(*tree_below);
    }
    let score_below = check_direction(current_tree, &below);

    return score_above * score_left * score_below * score_right;
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

    let mut high_score = 0;
    for (y_pos, row) in grid.iter().enumerate() {
        for (x_pos, _current_tree) in row.iter().enumerate() {
            let score = scenic_score(y_pos, x_pos, &grid);
            if score > high_score {
                high_score = score;
            }
        }
    }

    println!("High score {}", high_score);
}
