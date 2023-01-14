use std::{collections::BTreeMap, fs};

fn main() {
    let file_path = "day12_input.txt";
    let contents = fs::read_to_string(file_path).unwrap();

    let mut grid: Vec<Vec<u8>> = vec![];
    let mut start: (usize, usize) = (0, 0);
    let mut goals: Vec<(usize, usize)> = vec![];

    // Djikstras

    // "coordinate -> value" where value is current shortest length from the starting position
    let mut unvisited_nodes: BTreeMap<(usize, usize), Option<u32>> = BTreeMap::new();

    for (y, line) in contents.lines().enumerate() {
        grid.push(vec![]);

        for (x, character) in line.chars().enumerate() {
            if character == 'S' {
                grid[y].push('a' as u8);
                unvisited_nodes.insert((y, x), None);
                continue;
            }
            if character == 'E' {
                grid[y].push('z' as u8);
                start = (y, x);
                unvisited_nodes.insert((y, x), Some(0)); // Starting point gets value 0
                continue;
            }
            if character == 'a' {
                goals.push((y, x));
            }
            grid[y].push(character as u8);
            unvisited_nodes.insert((y, x), None);
        }
    }

    let shortest_path = dijkstra(start, goals, &grid, &mut unvisited_nodes).unwrap();
    println!("Shortest path: {}", shortest_path);
}

fn valid_path(
    target_node: (usize, usize),
    current_node: (usize, usize),
    unvisited_nodes: &mut BTreeMap<(usize, usize), Option<u32>>,
    grid: &Vec<Vec<u8>>,
) -> bool {
    let current_value = grid[current_node.0][current_node.1];
    let target_value = grid[target_node.0][target_node.1];

    // TODO: Remove?????????
    let is_same_as_current = target_node == current_node;
    let is_unvisited = match unvisited_nodes.get(&target_node) {
        Some(_) => true,
        None => false,
    };
    let is_walkable = current_value == target_value || {
        if current_value > target_value {
            current_value - target_value == 1
        } else {
            target_value - current_value >= 1
        }
    };

    return is_walkable && !is_same_as_current && is_unvisited;
}

fn get_surrounding_nodes(
    node: (usize, usize),
    grid: &Vec<Vec<u8>>,
    unvisited_nodes: &mut BTreeMap<(usize, usize), Option<u32>>,
) -> Vec<(usize, usize)> {
    let y_cap = grid.len() - 1;
    let x_cap = grid[0].len() - 1;

    // current
    let y = node.0;
    let x = node.1;

    let mut surrounding_nodes: Vec<(usize, usize)> = vec![];

    // Left
    if x > 0 {
        let node_to_check = (y, x - 1);
        let is_valid_path = valid_path(node_to_check, (y, x), unvisited_nodes, grid);
        if is_valid_path {
            surrounding_nodes.push(node_to_check);
        }
    }
    // Right
    if x < x_cap {
        let node_to_check = (y, x + 1);
        let is_valid_path = valid_path(node_to_check, (y, x), unvisited_nodes, grid);
        if is_valid_path {
            surrounding_nodes.push(node_to_check);
        }
    }
    // Up
    if y > 0 {
        let node_to_check = (y - 1, x);
        let is_valid_path = valid_path(node_to_check, (y, x), unvisited_nodes, grid);
        if is_valid_path {
            surrounding_nodes.push(node_to_check);
        }
    }
    // Down
    if y < y_cap {
        let node_to_check = (y + 1, x);
        let is_valid_path = valid_path(node_to_check, (y, x), unvisited_nodes, grid);
        if is_valid_path {
            surrounding_nodes.push(node_to_check);
        }
    }

    return surrounding_nodes;
}

fn dijkstra(
    current_node: (usize, usize),
    goals: Vec<(usize, usize)>,
    grid: &Vec<Vec<u8>>,
    unvisited_nodes: &mut BTreeMap<(usize, usize), Option<u32>>,
) -> Option<u32> {
    let surrounding_nodes = get_surrounding_nodes(current_node, grid, unvisited_nodes);
    let current_node_value = unvisited_nodes.get(&current_node).unwrap().unwrap();

    // When we find the target
    if goals.contains(&current_node) {
        return Some(current_node_value);
    }

    for sn in &surrounding_nodes {
        let sn_value = unvisited_nodes.get(&sn).unwrap();
        match sn_value {
            Some(val) => {
                if current_node_value + 1 < *val {
                    unvisited_nodes.insert(*sn, Some(current_node_value + 1));
                };
            }
            None => {
                unvisited_nodes.insert(*sn, Some(current_node_value + 1));
            }
        }
    }

    unvisited_nodes.remove(&current_node);

    // Sort unvisited_nodes in order of smallest value first
    let mut unvisited_sorted: Vec<(&(usize, usize), &Option<u32>)> = unvisited_nodes
        .iter()
        .filter(|x| match x.1 {
            Some(_) => true,
            None => false,
        })
        .collect();
    unvisited_sorted.sort_by(|a, b| a.1.cmp(b.1));

    let next_unvisited_node = unvisited_sorted[0];
    let next_node = next_unvisited_node.0;

    return dijkstra(*next_node, goals, grid, unvisited_nodes);
}
