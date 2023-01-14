use std::{collections::BTreeMap, fs};

fn main() {
    let file_path = "day12_input.txt";
    let contents = fs::read_to_string(file_path).unwrap();

    let mut grid: Vec<Vec<u8>> = vec![];
    let mut start: (usize, usize) = (0, 0);
    let mut goal: (usize, usize) = (0, 0);

    // Djikstras

    // "coordinate -> value" where value is current shortest length from the starting position
    let mut unvisited_nodes: BTreeMap<(usize, usize), Option<u32>> = BTreeMap::new();

    for (y, line) in contents.lines().enumerate() {
        grid.push(vec![]);

        for (x, character) in line.chars().enumerate() {
            if character == 'S' {
                grid[y].push('a' as u8);
                start = (y, x);
                unvisited_nodes.insert((y, x), Some(0)); // Starting point gets value 0
                continue;
            }
            if character == 'E' {
                grid[y].push('z' as u8);
                goal = (y, x);
                unvisited_nodes.insert((y, x), None);
                continue;
            }
            unvisited_nodes.insert((y, x), None);
            grid[y].push(character as u8);
        }
    }

    let shortest_path = dijkstra(start, goal, &grid, &mut unvisited_nodes).unwrap();
    println!("Shortest path: {}", shortest_path);
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
    let current = grid[y][x];

    let mut surrounding_nodes: Vec<(usize, usize)> = vec![];

    // Left
    if x > 0 {
        let val = (y, x - 1);
        let is_same_as_current = (y + 1, x) == (y, x);
        let is_unvisited = match unvisited_nodes.get(&val) {
            Some(_) => true,
            None => false,
        };
        let is_walkable = current >= grid[val.0][val.1] - 1;
        if is_walkable && !is_same_as_current && is_unvisited {
            surrounding_nodes.push(val);
        }
    }
    // Right
    if x < x_cap {
        let val = (y, x + 1);
        let is_same_as_current = (y + 1, x) == (y, x);
        let is_unvisited = match unvisited_nodes.get(&val) {
            Some(_) => true,
            None => false,
        };
        let is_walkable = current >= grid[val.0][val.1] - 1;
        if is_walkable && !is_same_as_current && is_unvisited {
            surrounding_nodes.push(val);
        }
    }
    // Up
    if y > 0 {
        let val = (y - 1, x);
        let is_same_as_current = (y + 1, x) == (y, x);
        let is_unvisited = match unvisited_nodes.get(&val) {
            Some(_) => true,
            None => false,
        };
        let is_walkable = current >= grid[val.0][val.1] - 1;
        if is_walkable && !is_same_as_current && is_unvisited {
            surrounding_nodes.push(val);
        }
    }
    // Down
    if y < y_cap {
        let val = (y + 1, x);
        let is_same_as_current = (y + 1, x) == (y, x);
        let is_unvisited = match unvisited_nodes.get(&val) {
            Some(_) => true,
            None => false,
        };
        let is_walkable = current >= grid[val.0][val.1] - 1;
        if is_walkable && !is_same_as_current && is_unvisited {
            surrounding_nodes.push(val);
        }
    }

    return surrounding_nodes;
}

fn dijkstra(
    current_node: (usize, usize),
    goal: (usize, usize),
    grid: &Vec<Vec<u8>>,
    unvisited_nodes: &mut BTreeMap<(usize, usize), Option<u32>>,
) -> Option<u32> {
    let surrounding_nodes = get_surrounding_nodes(current_node, grid, unvisited_nodes);
    let current_node_value = unvisited_nodes.get(&current_node).unwrap().unwrap();

    // When we find the target
    if current_node == goal {
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

    println!("current node {:?}", current_node);
    println!("current value {:?}", current_node_value);
    println!("surrounding {:?}", surrounding_nodes);

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
    let next_node_value = next_unvisited_node.1;
    println!("next_node {:?}, {:?}", next_node, next_node_value);
    return dijkstra(*next_node, goal, grid, unvisited_nodes);
}
